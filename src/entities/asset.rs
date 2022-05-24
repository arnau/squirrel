use serde::{Deserialize, Serialize};
use super::Pyramid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Blob {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug)]
pub enum BlobSize {
    Max,
    Thumbnail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub metadata: AssetMetadata,
    pub blob: Blob,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetMetadata {
    pub rating: Option<usize>,
    pub flag: Option<bool>,
    pub label: Option<String>,
    pub format: String,
    pub width: usize,
    pub height: usize,
    pub orientation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetRow {
    pub id: String,
    pub pyramid: Pyramid,
    pub metadata: AssetMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thumbnail {
    pub id: String,
    pub path: String,
    pub blob: Blob,
}
