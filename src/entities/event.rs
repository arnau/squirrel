use crate::{Version, VERSION};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

/// Represents an event that happened in the system. For example an import action started.
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub squirrel_version: Version,
    pub stamp: DateTime<Utc>,
    pub action: String,
    pub bag: serde_json::Value,
}

impl Event {
    pub fn new(action: &str, bag: serde_json::Value) -> Self {
        let stamp = Utc::now();

        Event {
            action: action.to_string(),
            squirrel_version: VERSION,
            stamp,
            bag,
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let blob = serde_json::to_string(self).expect("event to be serialisable as json");

        write!(f, "{}", blob)
    }
}

impl FromStr for Event {
    type Err = ParseEventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let event: Event = serde_json::from_str(s).map_err(|e| ParseEventError(e.to_string()))?;

        Ok(event)
    }
}

#[derive(Debug, Error)]
#[error("Could not parse the event: {0}")]
pub struct ParseEventError(String);
