use crate::entities::Result;
use anyhow::anyhow as ah;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Represents a Lightroom source.
#[derive(Debug, Clone)]
pub struct Source {
    pub name: String,
    pub path: String,
    pub version: usize,
}

impl Source {
    /// Returns the path to the `.lrcat` database.
    ///
    /// ## Exceptions
    ///
    /// Returns a [`SourceError::MissingFile`] when the `.lrcat` can't be found.
    pub fn catalogue_db_path(&self) -> Result<PathBuf> {
        let path = Path::new(&self.path)
            .join(&self.name)
            .with_extension("lrcat");
        if !&path.is_file() {
            return Err(ah!(SourceError::MissingFile(path.to_string_lossy().into())));
        }

        Ok(path)
    }

    /// Returns the path to the previews.
    ///
    /// ## Exceptions
    ///
    /// Returns a [`SourceError::MissingDir`] when the previews directory can't be found.
    pub fn previews_path(&self) -> Result<PathBuf> {
        let name = format!("{} Previews.lrdata/", &self.name);
        let path = Path::new(&self.path).join(name);
        if !&path.exists() {
            return Err(ah!(SourceError::MissingDir(path.to_string_lossy().into())));
        }

        Ok(path)
    }

    /// Returns the path to the `previews.db` database.
    ///
    /// ## Exceptions
    ///
    /// Returns a [`SourceError::MissingFile`] when the `previews.db` can't be found.
    pub fn previews_db_path(&self) -> Result<PathBuf> {
        let name = format!("{} Previews.lrdata/previews.db", &self.name);
        let path = Path::new(&self.path).join(name);
        if !&path.is_file() {
            return Err(ah!(SourceError::MissingFile(path.to_string_lossy().into())));
        }

        Ok(path)
    }

    /// Returns the path to the `root-pixels.db` database.
    ///
    /// ## Exceptions
    ///
    /// Returns a [`SourceError::MissingFile`] when the `root-pixels.db` can't be found.
    pub fn rootpixels_db_path(&self) -> Result<PathBuf> {
        let name = format!("{} Previews.lrdata/root-pixels.db", &self.name);
        let path = Path::new(&self.path).join(name);
        if !&path.is_file() {
            return Err(ah!(SourceError::MissingFile(path.to_string_lossy().into())));
        }

        Ok(path)
    }

    /// Returns the path to the `helper.db` database.
    ///
    /// ## Exceptions
    ///
    /// Returns a [`SourceError::MissingFile`] when the `helper.db` can't be found.
    pub fn helper_db_path(&self) -> Result<PathBuf> {
        let name = format!("{} Helper.lrdata/helper.db", &self.name);
        let path = Path::new(&self.path).join(name);
        if !&path.is_file() {
            return Err(ah!(SourceError::MissingFile(path.to_string_lossy().into())));
        }

        Ok(path)
    }
}

#[derive(Error, Debug)]
pub enum SourceError {
    #[error("File not found: {0}")]
    MissingFile(String),
    #[error("Dir not found: {0}")]
    MissingDir(String),
    #[error("Unknown Lightroom version {0}")]
    UnknownVersion(usize),
}
