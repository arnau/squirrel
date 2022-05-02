use crate::entities::Result;
use anyhow::anyhow as ah;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Source {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub version: String,
}

impl Source {
    pub fn try_new(id: String, version: String, spath: &str) -> Result<Self> {
        let path = PathBuf::from_str(spath)?;
        let name = path
            .file_stem()
            .ok_or(ah!(SourceError::NotAFile(spath.to_string())))?
            .to_str()
            .ok_or(ah!(SourceError::InvalidString))?
            .to_string();

        if let Some(extension) = path.extension() {
            if extension != "lrcat" {
                return Err(ah!(SourceError::UnexpectedExtension));
            }
        } else {
            return Err(ah!(SourceError::MissingExtension));
        };

        let parent = path
            .parent()
            .expect("Parent directory to exist.")
            .to_path_buf();

        let source = Source {
            id,
            name,
            path: parent,
            version,
        };

        Ok(source)
    }

    /// Returns the path to the `.lrcat` database.
    pub fn catalogue_path(&self) -> PathBuf {
        self.path.join(&self.name).with_extension("lrcat")
    }

    /// Returns the path to the `previews.db` database.
    pub fn previews_path(&self) -> PathBuf {
        let name = format!("{} Previews.lrdata/previews.db", &self.name);
        self.path.join(name)
    }

    /// Returns the path to the `root-pixels.db` database.
    pub fn rootpixels_path(&self) -> PathBuf {
        let name = format!("{} Previews.lrdata/root-pixels.db", &self.name);
        self.path.join(name)
    }

    /// Returns the path to the `helper.db` database.
    pub fn helper_path(&self) -> PathBuf {
        let name = format!("{} Helper.lrdata/helper.db", &self.name);
        self.path.join(name)
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

impl fmt::Display for SourceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SourceKind::Lightroom => write!(f, "lightroom"),
        }
    }
}

#[derive(Error, Debug)]
pub enum SourceError {
    #[error("Unknown source kind. No extension found.")]
    MissingExtension,
    #[error("The given source path is not a valid string for this operating system.")]
    InvalidString,
    #[error("Unknown source kind. The file extension must be 'lrcat'.")]
    UnexpectedExtension,
    #[error("The given source path is not a file.")]
    NotAFile(String),
}
