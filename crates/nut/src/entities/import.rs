use std::path::PathBuf;
use std::str::FromStr;
use thiserror::Error;
use crate::entities::Result;
use anyhow::anyhow as ah;

/// Represents a Lightroom to be imported as a [`crate::entities::Source`].
#[derive(Debug, Clone)]
pub struct Import {
    pub name: String,
    pub path: PathBuf,
}

impl Import {
    pub fn try_new(spath: &str) -> Result<Self> {
        let path = PathBuf::from_str(spath)?;
        let name = path
            .file_stem()
            .ok_or_else(|| ah!(ImportError::NotAFile(spath.to_string())))?
            .to_str()
            .ok_or_else(|| ah!(ImportError::InvalidOsPath))?
            .to_string();

        if let Some(extension) = path.extension() {
            if extension != "lrcat" {
                return Err(ah!(ImportError::UnexpectedExtension));
            }
        } else {
            return Err(ah!(ImportError::MissingExtension));
        };

        let parent = path
            .parent()
            .expect("Parent directory to exist.")
            .to_path_buf();

        let source = Import { name, path: parent };

        Ok(source)
    }

    /// Returns the path to the `.lrcat` database.
    ///
    /// ## Exceptions
    ///
    /// Returns a [`ImportError::MissingFile`] when the `.lrcat` can't be found.
    pub fn catalogue_path(&self) -> Result<PathBuf> {
        let path = self.path.join(&self.name).with_extension("lrcat");
        if !&path.is_file() {
            return Err(ah!(ImportError::MissingFile(path.to_string_lossy().into())));
        }

        Ok(path)
    }

    /// Returns the path to the `previews.db` database.
    ///
    /// ## Exceptions
    ///
    /// Returns a [`ImportError::MissingFile`] when the `previews.db` can't be found.
    pub fn previews_path(&self) -> Result<PathBuf> {
        let name = format!("{} Previews.lrdata/previews.db", &self.name);
        let path = self.path.join(name);
        if !&path.is_file() {
            return Err(ah!(ImportError::MissingFile(path.to_string_lossy().into())));
        }

        Ok(path)
    }

    /// Returns the path to the `root-pixels.db` database.
    ///
    /// ## Exceptions
    ///
    /// Returns a [`ImportError::MissingFile`] when the `root-pixels.db` can't be found.
    pub fn rootpixels_path(&self) -> Result<PathBuf> {
        let name = format!("{} Previews.lrdata/root-pixels.db", &self.name);
        let path = self.path.join(name);
        if !&path.is_file() {
            return Err(ah!(ImportError::MissingFile(path.to_string_lossy().into())));
        }

        Ok(path)
    }

    /// Returns the path to the `helper.db` database.
    ///
    /// ## Exceptions
    ///
    /// Returns a [`ImportError::MissingFile`] when the `helper.db` can't be found.
    pub fn helper_path(&self) -> Result<PathBuf> {
        let name = format!("{} Helper.lrdata/helper.db", &self.name);
        let path = self.path.join(name);
        if !&path.is_file() {
            return Err(ah!(ImportError::MissingFile(path.to_string_lossy().into())));
        }

        Ok(path)
    }
}

#[derive(Error, Debug)]
pub enum ImportError {
    #[error("Unknown source kind. No extension found.")]
    MissingExtension,
    #[error("The given source path is not a valid string for this operating system.")]
    InvalidOsPath,
    #[error("Unknown source kind. The file extension must be 'lrcat'.")]
    UnexpectedExtension,
    #[error("The given source path is not a file: {0}")]
    NotAFile(String),
    #[error("File not found: {0}")]
    MissingFile(String),
    #[error("Unknown Lightroom version {0}")]
    UnknownVersion(usize),
    #[error("Source already exists")]
    SourceExists,
}
