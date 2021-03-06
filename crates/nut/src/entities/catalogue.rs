use thiserror::Error;
use crate::Version;

/// Represents a Squirrel catalogue
#[derive(Debug, Clone)]
pub struct Catalogue {
    pub version: Version,
    // pub path: PathBuf,
}

#[derive(Error, Debug)]
pub enum CatalogueError {
    #[error("The catalogue version {0} is incompatible with the current version of Squirrel.")]
    IncompatibleVersion(String),
}
