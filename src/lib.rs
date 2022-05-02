pub mod entities;
pub mod pyramid;
pub mod repositories;
pub mod services;

use walkdir::DirEntry;

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
