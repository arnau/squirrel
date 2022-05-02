use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Source {
    pub id: String,
    pub name: String,
    pub kind: SourceKind,
    pub path: PathBuf,
}

impl FromStr for Source {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let path = PathBuf::from_str(s)?;
        let name = path
            .file_stem()
            .ok_or(anyhow::anyhow!("The given source path is not a file."))?
            .to_str()
            .ok_or(anyhow::anyhow!(
                "The given source path is not a valid string for this operating system."
            ))?
            .to_string();
        let id = if let Some((id, _)) = name.rsplit_once('-') {
            id.to_string()
        } else {
            name.clone()
        };
        let kind = if let Some(extension) = path.extension() {
            SourceKind::from_str(extension.to_str().expect("extension to be a string."))?
        } else {
            anyhow::bail!("Unknown source. No extension found.");
        };

        let source = Source {
            id,
            name,
            kind,
            path,
        };

        Ok(source)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SourceKind {
    Lightroom,
}

impl FromStr for SourceKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "lightroom" => Ok(SourceKind::Lightroom),
            "lrcat" => Ok(SourceKind::Lightroom),
            _ => Err(anyhow::anyhow!("Unknown source kind")),
        }
    }
}
