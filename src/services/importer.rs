use crate::entities::{storage::Pool, Result};
use crate::repositories::import::ImportRepository;
use anyhow::anyhow as ah;
use std::path::PathBuf;
use std::str::FromStr;
use thiserror::Error;

/// Attempts to import the information contained in the given Lightroom catalogue.
///
/// The given path must be a `.lrcat` catalogue. It assumes a layout structure like:
///
/// ```text
/// foo/
///   foo.lrcat
///   foo Helper.lrdata/
///     helper.db
///   foo Previews.lrdata/
///     previews.db
///     root-pixels.db
/// ```
pub fn import(pool: &Pool, path: &str) -> Result<()> {
    let source = Import::try_new(path)?;
    let mut conn = pool.get()?;

    dbg!(&source);

    ImportRepository::attach_catalogue(&conn, &source)?;

    let tx = conn.transaction()?;

    let version = ImportRepository::version(&tx)?;

    if !(version >= 11_00_00_0 && version < 12_00_00_0) {
        return Err(ah!(ImportError::UnknownVersion(version)));
    }

    ImportRepository::insert_source(&tx, &source)?;
    ImportRepository::copy_roots(&tx)?;
    ImportRepository::copy_folders(&tx)?;
    ImportRepository::copy_files(&tx)?;
    ImportRepository::copy_assets(&tx)?;

    tx.commit()?;

    let mut previews_path = source.previews_path()?;
    previews_path.pop();

    let broken_pyramids = ImportRepository::check_broken_pyramids(&conn, &previews_path)?;

    dbg!(broken_pyramids);

    ImportRepository::detach_catalogue(&conn)?;

    Ok(())
}

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
            .ok_or(ah!(ImportError::NotAFile(spath.to_string())))?
            .to_str()
            .ok_or(ah!(ImportError::InvalidOsPath))?
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
    pub fn catalogue_path(&self) -> Result<PathBuf> {
        let path = self.path.join(&self.name).with_extension("lrcat");
        if !&path.is_file() {
            return Err(ah!(ImportError::MissingFile(path.to_string_lossy().into())));
        }

        Ok(path)
    }

    /// Returns the path to the `previews.db` database.
    pub fn previews_path(&self) -> Result<PathBuf> {
        let name = format!("{} Previews.lrdata/previews.db", &self.name);
        let path = self.path.join(name);
        if !&path.is_file() {
            return Err(ah!(ImportError::MissingFile(path.to_string_lossy().into())));
        }

        Ok(path)
    }

    /// Returns the path to the `root-pixels.db` database.
    pub fn rootpixels_path(&self) -> Result<PathBuf> {
        let name = format!("{} Previews.lrdata/root-pixels.db", &self.name);
        let path = self.path.join(name);
        if !&path.is_file() {
            return Err(ah!(ImportError::MissingFile(path.to_string_lossy().into())));
        }

        Ok(path)
    }

    /// Returns the path to the `helper.db` database.
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
}
