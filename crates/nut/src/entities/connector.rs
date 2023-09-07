use rusqlite::{Result as SQLResult, Row};
use serde::{Deserialize, Serialize};
// use chrono::{DateTime, Utc};


pub type ConnectorId = String;


/// Represents a Connector configuration. E.g. BackBlaze
///
/// ## Example
///
/// ```rust
/// let connector = Connector {
///     id: "6ad327d8a3d112847a520f17".into(),
///     key_name: "Squirrel-Keyname".into(),
///     bucket_name: "Squirrel-Bucketname".into(),
///     secret_key: true,
///     kind: "backblaze".into(),
///     creation_stamp: "2023-06-12T12:13:14Z".into(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connector {
    pub id: ConnectorId,
    pub key_name: String,
    pub bucket_name: String,
    pub secret_key: bool,
    pub kind: String,
    pub creation_stamp: String, // TODO DateTime<Utc>,
}

impl Connector {
    pub fn from_row(row: &Row<'_>) -> SQLResult<Self> {
        let id: ConnectorId = row.get(0)?;
        let key_name: String = row.get(1)?;
        let bucket_name: String = row.get(2)?;
        let secret_key: bool = row.get(3)?;
        let kind: String = row.get(4)?;
        let creation_stamp: String = row.get(5)?;

        let connector = Self {
            id,
            key_name,
            bucket_name,
            secret_key,
            kind,
            creation_stamp,
        };

        Ok(connector)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewConnector {
    pub id: ConnectorId,
    pub key_name: String,
    pub bucket_name: String,
    pub secret_key: String,
    pub kind: String,
}
