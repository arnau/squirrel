mod header;
mod parser;
pub use parser::extract;

/// Represents an image pyramid.
#[derive(Debug, Clone)]
pub struct Pyramid {
    pub color_profile: String,
    pub cropped_height: u32,
    pub cropped_width: u32,
    pub digest: String,
    pub file_timestamp: u32,
    pub format_version: u32,
    pub from_proxy: bool,
    pub levels: Vec<Level>,
    pub quality: String,
    pub uuid: String,
}

/// Represents an image pyramid level combining the header metadata and the JPEG data.
#[derive(Debug, Clone)]
pub struct Level {
    pub height: u32,
    pub width: u32,
    pub blob: Vec<u8>,
}
