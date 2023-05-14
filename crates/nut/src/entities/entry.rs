use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use rusqlite::{Result as SQLResult, Row};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type EntryId = String;
pub type EntryPath = String;

/// Represents a Squirrel entry. Either a Folder or a File.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub id: EntryId,
    pub path: EntryPath,
    pub kind: Kind,
    pub parent_id: Option<EntryId>,
    pub root_id: String,
    pub source_id: String,
}

impl Entry {
    pub fn from_row(row: &Row<'_>) -> SQLResult<Entry> {
        let id: EntryId = row.get(0)?;
        let path: EntryPath = row.get(1)?;
        let kind: Kind = row.get(2)?;
        let parent_id: Option<EntryId> = row.get(3)?;
        let root_id: String = row.get(4)?;
        let source_id: String = row.get(5)?;

        let entry = Entry {
            id,
            path,
            kind,
            parent_id,
            root_id,
            source_id,
        };

        Ok(entry)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    File,
    Folder,
}

impl FromSql for Kind {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).and_then(|s| match s.as_str() {
            "file" => Ok(Kind::File),
            "folder" => Ok(Kind::Folder),
            _ => Err(FromSqlError::Other(Box::new(EntryError::UnknownKind(s)))),
        })
    }
}

impl ToSql for Kind {
    fn to_sql(&self) -> SQLResult<ToSqlOutput> {
        let s = match self {
            Kind::File => "file",
            Kind::Folder => "folder",
        };

        Ok(s.into())
    }
}

#[derive(Error, Debug)]
pub enum EntryError {
    #[error("{0} is not a known kind of entry.")]
    UnknownKind(String),
}
