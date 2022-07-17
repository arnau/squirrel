use crate::entities::entry::Kind;
use serde::{Deserialize, Serialize};
use std::fmt;
use super::asset::{AssetMetadata, AssetId};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename = "snake_case")]
pub enum Stem {
    Asset {
        id: AssetId,
        path: String,
        master_id: Option<AssetId>,
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
            Stem::Asset { id, .. } => id,
            Stem::Folder { id, .. } => id,
        }
    }

    pub fn path(&self) -> &str {
        match self {
            Stem::Asset { path, .. } => path,
            Stem::Folder { path, .. } => path,
        }
    }
}

impl fmt::Display for Stem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stem::Asset { path, .. } => write!(f, "{}", path),
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

