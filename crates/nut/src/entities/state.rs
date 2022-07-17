use super::{
    asset::{AssetId, AssetMetadata},
    location::Location,
};
use serde::{Deserialize, Serialize};

/// Represents the UI state for a given location.
///
///
/// ## Singularity: the "home", where no root is selected. aka "/"
///
/// ```nocode
/// State::Catalogue {
///     location: vec![], // empty
///     roots: vec![...], // all roots
///     folders: vec![], // empty
///     assets: vec![], // empty
/// }
/// ```
///
/// ## Root: e.g. "/2021/"
///
/// ```nocode
/// State::Catalogue {
///     location: vec![Stem::Folder { id: "xxx", path: "/2021/" }], // empty
///     roots: vec!["/2021/", "/2022/"], // all roots
///     folders: vec![], // empty
///     assets: vec![], // empty
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum State {
    /// A state for the catalogue.
    Catalogue {
        // The data for the current path
        location: Location,
        // All possible roots.
        roots: Vec<Folder>,
        // The folders hanging from the location root.
        folders: Vec<Folder>,
        // The assets under the selected folder. Thumbnails are retrieved independently.
        assets: Vec<Asset>,
    },

    /// A state for the config.
    Config {},
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Folder {
    pub id: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    pub id: AssetId,
    pub path: String,
    pub master_id: Option<AssetId>,
    pub metadata: AssetMetadata,
}
