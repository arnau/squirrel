use crate::entities::entry::Kind;
use serde::{Deserialize, Serialize};
use std::fmt;
use super::asset::AssetMetadata;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename = "snake_case")]
pub enum Stem {
    File {
        id: String,
        path: String,
        metadata: AssetMetadata,
    },
    Folder {
        id: String,
        path: String,
    },
}

impl Stem {
    pub fn id(&self) -> &str {
        match self {
            Stem::File { id, .. } => id,
            Stem::Folder { id, .. } => id,
        }
    }

    pub fn path(&self) -> &str {
        match self {
            Stem::File { path, .. } => path,
            Stem::Folder { path, .. } => path,
        }
    }
}

impl fmt::Display for Stem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stem::File { path, .. } => write!(f, "{}", path),
            Stem::Folder { path, .. } => write!(f, "{}", path),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StemRow {
    pub id: String,
    pub path: String,
    pub kind: Kind,
}

