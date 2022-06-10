use super::stem::Stem;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the hierarchy from the root to the current location.
#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    /// Either the same as the last stem path or "/"
    pub path: String,
    pub stems: Vec<Stem>,
}

impl Location {
    pub fn new(stems: Vec<Stem>) -> Self {
        let path = if let Some(stem) = stems.last() {
            stem.path().to_string()
        } else {
            // Must be root.
            "/".to_string()
        };

        Location { path, stems }
    }

    pub fn inner(&self) -> &[Stem] {
        &self.stems
    }

    pub fn parent(&self) -> Option<&Stem> {
        self.stems.get(self.stems.len() - 2)
    }

    pub fn ancestor(&self) -> Option<&Stem> {
        let len = self.stems.len();
        if let Some(current) = self.stems.last() {
            match current {
                Stem::File { .. } => {
                    match len {
                        // Files cannot exist alone.
                        0..=1 => unreachable!(),
                        // Most files are under a folder or subfolder.
                        _ => self.stems.get(len - 2),
                    }
                }
                Stem::Folder { .. } => {
                    match len {
                        0 => None,
                        // A root folder.
                        1 => self.stems.get(0),
                        _ => self.stems.get(len - 1),
                    }
                }
            }
        } else {
            None
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(stem) = self.stems.last() {
            write!(f, "{}", stem)
        } else {
            write!(f, "/")
        }
    }
}
