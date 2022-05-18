use lazy_static::lazy_static;
use walkdir::DirEntry;

pub mod entities;
pub mod functions;
pub mod pyramid;
pub mod repositories;
pub mod services;

// Version decomposes as:
//
// 1. Major
// 2. Minor
// 3. Patch
pub const VERSION: [u16; 3] = [0, 1, 0];

lazy_static! {
    static ref BOOTSTRAP: &'static str = include_str!("./storage/catalogue.sql");
}

// TODO: Re-organise
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
