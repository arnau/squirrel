use super::asset::{AssetId, AssetMetadata};
use serde::{Deserialize, Serialize};

pub type FolderId = String;
pub type FolderPath = String;

// Should always be ""
pub type GroundId = String;

/// A UI catalogue entry
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Location {
    Ground {
        id: GroundId,
    },
    Folder {
        id: FolderId,
        path: FolderPath,
        trail: Vec<FolderId>,
    },
    Asset {
        id: AssetId,
        path: AssetPath,
        trail: Vec<FolderId>,
    },
}

/// The list of folders for a location.
#[derive(Debug, Serialize, Deserialize)]
pub struct LocationFolders(pub Vec<FolderEntry>);

/// A page of assets for a location.
#[derive(Debug, Serialize, Deserialize)]
pub struct LocationAssetPage {
    /// When the cursor is `Option::None`, it's the last page.
    pub next_cursor: Option<AssetCursor>,
    pub data: Vec<Asset>,
}

/// The list of roots for the catalogue.
#[derive(Debug, Serialize, Deserialize)]
pub struct Ground(pub Vec<FolderEntry>);

/// A UI tree item.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FolderEntry {
    pub id: FolderId,
    pub path: FolderPath,
    pub counter: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FolderDetails {
    pub id: FolderId,
    pub path: FolderPath,
    pub source: Source,
    pub root: Root,
    pub folder_count: usize,
    pub asset_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Source {
    pub id: String,
    pub name: String,
    pub path: String,
    pub version: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Root {
    pub id: String,
    pub name: String,
    pub path: String,
}

/// Asset path is a combination of "{entry.path}+{master_id}".
pub type AssetPath = String;

/// A cursor for asset pages based on `AssetPath`. See `StateRepository::get_asset_page`
pub type AssetCursor = AssetPath;

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    pub id: AssetId,
    pub path: AssetPath,
    pub master_id: Option<AssetId>,
    pub metadata: AssetMetadata,
}
