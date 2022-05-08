use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Source {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub version: String,
}
