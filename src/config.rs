use std::path::PathBuf;

use directories::ProjectDirs;

pub(crate) const QUALIFIER: &'static str = "net";
pub(crate) const ORGANIZATION: &'static str = "seachess";
pub(crate) const APPLICATION: &'static str = "squirrel";
pub(crate) const DB_NAME: &'static str = "squirrel.db";

/// Linux:   /home/alice/.config/squirrel
/// Windows: C:\Users\Alice\AppData\Roaming\seachess\squirrel
/// macOS:   /Users/Alice/Library/Application Support/net.seachess.squirrel
pub fn directories() -> ProjectDirs {
    ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION).expect("project directory")
}

pub fn db_path() -> PathBuf {
    directories().data_dir().join(DB_NAME)
}
