//! This module implements a basic parser to extract all information packed into an Adobe Lightroom v11 Preview (`.lrprev`) image
//! pyramid.
//!
//! The expected binary structure is as follows:
//!
//! ```text
//! <signature = "AgHg"> <size> <label = "header">
//! <data>
//! <signature = "AgHg"> <size> <label = "level_1">
//! <data>
//! …
//! <signature = "AgHg"> <size> <label = "level_4">
//! <data>
//! …
//! EOF
//! ```
//!
//! The header section contains a data structure resembling a Lua table describing the content of
//! the pyramid. The amount of levels will vary depending on the image original size.
//!
//! ```lua
//! pyramid = {
//! 	colorProfile = "AdobeRGB",
//! 	croppedHeight = 4912,
//! 	croppedWidth = 7360,
//! 	digest = "030b36e11e9d722fdab20884884e0ff2",
//! 	fileTimeStamp = 645111123,
//! 	formatVersion = 3,
//! 	fromProxy = false,
//! 	levels = {
//! 		{
//! 			height = 61,
//! 			width = 90,
//! 		},
//! 		{
//! 			height = 121,
//! 			width = 180,
//! 		},
//! 		{
//! 			height = 241,
//! 			width = 360,
//! 		},
//! 	},
//! 	quality = "standard",
//! 	uuid = "FF4ADF67-3C63-4EB7-85B1-6D4409B537D3",
//! }
//! ```

use byteorder::{BigEndian, ReadBytesExt};
use std::collections::HashMap;
use std::io::prelude::*;

use super::header::{self, Header, Level};

/// Represents an image pyramid.
#[derive(Debug, Clone)]
pub struct PyramidObject {
    pub header: Option<Header>,
    pub blobs: HashMap<String, Vec<u8>>,
}

impl PyramidObject {
    pub fn len(&self) -> usize {
        self.blobs.len()
    }

    pub fn levels(&self) -> Vec<Level> {
        self.header.clone().expect("pyramid header to exist").levels
    }

    pub fn level(&self, number: usize) -> Level {
        self.header.clone().expect("pyramid header to exist").levels[number - 1]
    }
}

/// Any valid ".lrprev" file starts with "AgHg".
pub static MAGIC_LRPREV: &[u8; 4] = b"AgHg";

pub fn extract<R>(mut reader: R) -> anyhow::Result<PyramidObject>
where
    R: Read,
{
    let mut blobs: HashMap<String, Vec<u8>> = HashMap::new();
    let mut header: Option<Header> = None;

    loop {
        let mut buf = [0; 8];

        match reader.read(&mut buf[..]) {
            // end of stream
            Ok(0) => {
                break;
            }
            Ok(_) => {
                if &buf[0..4] == MAGIC_LRPREV {
                    // calculate number of bytes in the current section.
                    let size = reader.read_u64::<BigEndian>()?;
                    let n = (size as f64 / 8f64).ceil() as u64;
                    let mut buffer = vec![];

                    // read padding bytes
                    reader.read(&mut buf[..])?;

                    // read label. Either "header" or "level_x".
                    reader.read(&mut buf[..])?;

                    // trim label from padding.
                    let section = std::string::String::from_utf8_lossy(&buf)
                        .trim_matches(char::from(0))
                        .to_string();

                    // collect data from section
                    for _ in 0..n {
                        reader.read(&mut buf[..])?;
                        buffer.extend_from_slice(&buf);
                    }

                    match section.as_ref() {
                        "header" => {
                            let metadata = std::string::String::from_utf8_lossy(&buffer);
                            header = header::decode(&metadata).ok();
                        }
                        // level_x where x is a number from 1 to n.
                        _ => {
                            blobs.insert(section, buffer);
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("{:?}", err);
                break;
            }
        }
    }

    let pyramid = PyramidObject { header, blobs };

    Ok(pyramid)
}
