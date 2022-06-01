//! This module defines a parser for the Pyramid header data.

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use combine::parser::char::{space, string};
use combine::parser::combinator::{no_partial, FnOpaque};
use combine::{stream::position, *};

pub type Item = HashMap<String, Value>;

#[derive(PartialEq, Debug)]
pub enum Value {
    Array(Vec<Item>),
    Scalar(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Array(_) => unimplemented!(),
            Value::Scalar(s) => write!(f, "{}", s),
        }
    }
}

fn raw_value<Input>() -> impl Parser<Input, Output = Value>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1(satisfy(|c| c != '\n' && c != ',' && c != '"'))
        .map(|v| Value::Scalar(v))
        .message("while parsing raw value")
}

fn quoted_value<Input>() -> impl Parser<Input, Output = Value>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    between(token('"'), token('"'), raw_value().skip(spaces()))
        .message("while parsing quoted value")
}

fn array<Input>() -> impl Parser<Input, Output = Value>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    between(
        token('{').skip(spaces()),
        token('}').skip(spaces()),
        many(array_item().skip(spaces())),
    )
    .map(|v| Value::Array(v))
    .message("while parsing array")
}

fn array_item<Input>() -> impl Parser<Input, Output = Item>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    between(
        token('{').skip(spaces()),
        token('}').skip(spaces()),
        properties(),
    )
    .skip(token(','))
    .message("while parsing array item")
}

fn value<Input>() -> FnOpaque<Input, Value>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    opaque!(no_partial(
        choice((array(), quoted_value(), raw_value())).message("while parsing value")
    ))
}

fn property<Input>() -> impl Parser<Input, Output = (String, Value)>
where
    Input: Stream<Token = char>,
{
    (
        many1(none_of("{}=, ".chars())).skip(spaces()),
        token('=').skip(spaces()),
        value().skip(token(',')),
    )
        .map(|(key, _, value)| (key, value))
        .message("while parsing property")
}

fn spaces<Input>() -> impl Parser<Input>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    skip_many(space())
}

fn properties<Input>() -> impl Parser<Input, Output = HashMap<String, Value>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many(property().skip(spaces()))
}

fn header<Input>() -> impl Parser<Input, Output = HashMap<String, Value>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        string("pyramid")
            .skip(spaces())
            .skip(token('='))
            .skip(spaces()),
        between(
            token('{').skip(spaces()),
            token('}').skip(spaces()),
            properties(),
        ),
    )
        .map(|(_, v)| v)
        .message("while parsing header")
}

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub color_profile: Option<String>,
    pub cropped_height: Option<u32>,
    pub cropped_width: Option<u32>,
    pub digest: String,
    pub file_timestamp: Option<u32>,
    pub format_version: Option<u32>,
    pub from_proxy: Option<bool>,
    pub levels: Vec<Level>,
    pub quality: Option<String>,
    pub uuid: String,
}

/// Represents an image pyramid level combining the header metadata and the JPEG data.
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Level {
    pub height: u32,
    pub width: u32,
}

/// Decodes a pyramid header into a `Header`.
pub fn decode(input: &str) -> anyhow::Result<Header> {
    let (output, _) = header()
        .easy_parse(position::Stream::new(input))
        .map_err(|err| anyhow::anyhow!("{}", err))?;

    let from_proxy = output
        .get("fromProxy")
        .map(|v| FromStr::from_str(&v.to_string()).expect("proxy value to be a boolean"));

    let levels = if let Value::Array(inner) = output.get("levels").expect("levels") {
        inner
            .into_iter()
            .map(|h| {
                Ok(Level {
                    height: h.get("height").expect("height").to_string().parse()?,
                    width: h.get("width").expect("width").to_string().parse()?,
                })
            })
            .collect::<anyhow::Result<_>>()?
    } else {
        unreachable![]
    };

    Ok(Header {
        color_profile: output.get("colorProfile").map(|v| v.to_string()),
        cropped_height: output
            .get("croppedHeight")
            .map(|v| v.to_string().parse().expect("height u32")),
        cropped_width: output
            .get("croppedWidth")
            .map(|v| v.to_string().parse().expect("width u32")),
        digest: output.get("digest").expect("digest").to_string(),
        file_timestamp: output
            .get("fileTimeStamp")
            .map(|v| v.to_string().parse().expect("timestamp u32")),
        format_version: output
            .get("formatVersion")
            .map(|v| v.to_string().parse().expect("version u32")),
        from_proxy,
        levels,
        quality: output.get("quality").map(|v| v.to_string()),
        uuid: output.get("uuid").expect("uuid").to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_quoted_property() {
        let input = r#"colorProfile = "AdobeRGB","#;
        let output = property().parse(input).unwrap();
        assert_eq!(
            output,
            (
                (
                    "colorProfile".to_string(),
                    Value::Scalar("AdobeRGB".to_string())
                ),
                ""
            )
        );
    }

    #[test]
    fn test_raw_property() {
        let input = r#"formatVersion = 3,"#;
        let output = property().parse(input).unwrap();
        assert_eq!(
            output,
            (
                ("formatVersion".to_string(), Value::Scalar("3".to_string())),
                ""
            )
        );
    }

    #[test]
    fn test_properties() {
        let input = r#"height = 61, width = 90,"#;
        let expected: HashMap<String, Value> = vec![
            ("height".to_string(), Value::Scalar("61".to_string())),
            ("width".to_string(), Value::Scalar("90".to_string())),
        ]
        .into_iter()
        .collect();

        let (output, _) = properties()
            .easy_parse(position::Stream::new(input))
            .unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn test_array_item() {
        let input = "{ height = 61, width = 90, },";
        let expected: HashMap<String, Value> = vec![
            ("height".to_string(), Value::Scalar("61".to_string())),
            ("width".to_string(), Value::Scalar("90".to_string())),
        ]
        .into_iter()
        .collect();

        let (output, _) = array_item()
            .easy_parse(position::Stream::new(input))
            .unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn test_array_item_tab() {
        let input = r#"{
 	height = 61,
 	width = 90,
},"#;
        let expected: HashMap<String, Value> = vec![
            ("height".to_string(), Value::Scalar("61".to_string())),
            ("width".to_string(), Value::Scalar("90".to_string())),
        ]
        .into_iter()
        .collect();

        let (output, _) = array_item()
            .easy_parse(position::Stream::new(input))
            .unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn test_array() {
        let input = r#"{
 	{
 			height = 61,
 			width = 90,
 		},
 		{
 			height = 121,
 			width = 180,
 		},

},"#;
        let expected = Value::Array(vec![
            vec![
                ("height".to_string(), Value::Scalar("61".to_string())),
                ("width".to_string(), Value::Scalar("90".to_string())),
            ]
            .into_iter()
            .collect::<HashMap<String, Value>>(),
            vec![
                ("height".to_string(), Value::Scalar("121".to_string())),
                ("width".to_string(), Value::Scalar("180".to_string())),
            ]
            .into_iter()
            .collect::<HashMap<String, Value>>(),
        ]);

        let (output, _) = array().easy_parse(position::Stream::new(input)).unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn test_header() {
        let input = r#"pyramid = {
    colorProfile = "AdobeRGB",
    fileTimeStamp = 645111123,
    levels = {
        {
            height = 61,
            width = 90,
        },
        {
            height = 121,
            width = 180,
        },
    },
}"#;
        let expected = vec![
            (
                "colorProfile".to_string(),
                Value::Scalar("AdobeRGB".to_string()),
            ),
            (
                "fileTimeStamp".to_string(),
                Value::Scalar("645111123".to_string()),
            ),
            (
                "levels".to_string(),
                Value::Array(vec![
                    vec![
                        ("height".to_string(), Value::Scalar("61".to_string())),
                        ("width".to_string(), Value::Scalar("90".to_string())),
                    ]
                    .into_iter()
                    .collect::<HashMap<String, Value>>(),
                    vec![
                        ("height".to_string(), Value::Scalar("121".to_string())),
                        ("width".to_string(), Value::Scalar("180".to_string())),
                    ]
                    .into_iter()
                    .collect::<HashMap<String, Value>>(),
                ]),
            ),
        ]
        .into_iter()
        .collect::<HashMap<String, Value>>();

        let (output, _) = header().easy_parse(position::Stream::new(input)).unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn test_decode() -> anyhow::Result<()> {
        let input = r#"pyramid = {
    colorProfile = "AdobeRGB",
    croppedHeight = 4912,
    croppedWidth = 7360,
    digest = "030b36e11e9d722fdab20884884e0ff2",
    fileTimeStamp = 645111123,
    formatVersion = 3,
    fromProxy = false,
    levels = {
        {
            height = 61,
            width = 90,
        },
        {
            height = 121,
            width = 180,
        },
        {
            height = 241,
            width = 360,
        },
    },
    quality = "standard",
    uuid = "FF4ADF67-3C63-4EB7-85B1-6D4409B537D3",
}"#;
        let expected = Header {
            color_profile: Some("AdobeRGB".into()),
            cropped_height: Some(4912),
            cropped_width: Some(7360),
            digest: "030b36e11e9d722fdab20884884e0ff2".into(),
            file_timestamp: Some(645111123),
            format_version: Some(3),
            from_proxy: Some(false),
            levels: vec![
                Level {
                    height: 61,
                    width: 90,
                },
                Level {
                    height: 121,
                    width: 180,
                },
                Level {
                    height: 241,
                    width: 360,
                },
            ],

            quality: Some("standard".into()),
            uuid: "FF4ADF67-3C63-4EB7-85B1-6D4409B537D3".into(),
        };

        let output = decode(input)?;

        assert_eq!(output, expected);

        Ok(())
    }
}
