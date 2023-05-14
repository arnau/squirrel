use super::{
    asset::{AssetId, AssetMetadata},
    entry::{EntryId, EntryPath},
};
use serde::{Deserialize, Serialize};

// TODO: either EntryId | AssetId
pub type LocationId = String;
// TODO: either EntryPath or EntryPath+AssetId (copies need a fake path)
pub type LocationPath = String;

/// A UI catalogue entry
#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub id: LocationId,
    pub path: LocationPath,
    // TODO: Trail must contain only ancestors. So it never has AssetId.
    pub trail: Vec<EntryId>,
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
    pub id: EntryId,
    pub path: EntryPath,
    pub counter: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FolderDetails {
    pub id: EntryId,
    pub path: EntryPath,
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
