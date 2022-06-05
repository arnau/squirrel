use super::stem::Stem;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the hierarchy from the root to the current location.
#[derive(Debug, Serialize, Deserialize)]
pub struct Location(pub Vec<Stem>);

impl Location {
    pub fn new(inner: Vec<Stem>) -> Self {
        Location(inner)
    }

    pub fn inner(&self) -> &[Stem] {
        &self.0
    }

    pub fn parent(&self) -> Option<&Stem> {
        self.0.get(self.0.len() - 2)
    }

    pub fn ancestor(&self) -> Option<&Stem> {
        let len = self.0.len();
        if let Some(current) = self.0.last() {
            match current {
                Stem::File { .. } => {
                    match len {
                        // Files cannot exist alone.
                        0..=1 => unreachable!(),
                        // Most files are under a folder or subfolder.
                        _ => self.0.get(len - 2),
                    }
                }
                Stem::Folder { .. } => {
                    match len {
                        0 => None,
                        // A root folder.
                        1 => self.0.get(0),
                        _ => self.0.get(len - 1),
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
        if let Some(stem) = self.0.last() {
            write!(f, "{}", stem)
        } else {
            write!(f, "/")
        }
    }
}
