use anyhow::Result;
use thiserror::Error;
use std::path::Path;

pub use rusqlite::{ params, Params };

pub type ConnectionManager = r2d2_sqlite::SqliteConnectionManager;
pub type Connection = r2d2::PooledConnection<ConnectionManager>;
pub type Pool = r2d2::Pool<ConnectionManager>;

/// Represents the data storage system repositories operate on.
pub struct Storage;

impl Storage {
    /// Creates a connection `Pool` for the given database.
    pub fn file<P>(path: P) -> Result<Pool>
    where
        P: AsRef<Path>,
    {
        let manager = ConnectionManager::file(path);

        Ok(r2d2::Pool::new(manager)?)
    }

    pub fn get_one<T, P, F>(conn: Connection, query: &str, p: P, f: F) -> Result<T>
    where
        P: Params,
        F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
        T: std::fmt::Debug,
    {
        let mut stmt = conn.prepare(query)?;
        let mut rows = stmt.query(p)?.mapped(f);

        match rows.next() {
            Some(value) => Ok(value?),
            None => anyhow::bail!(StorageError::EmptySet(query.to_string())),
        }
    }
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("The query `{0}` returned an empty set.")]
    EmptySet(String),
}
