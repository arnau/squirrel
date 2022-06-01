use anyhow::Result;
use std::ops::Deref;
use std::path::Path;
use thiserror::Error;

pub use rusqlite::{params, Params, Transaction, Connection};

pub type ConnectionManager = r2d2_sqlite::SqliteConnectionManager;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager>;
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

    pub fn memory() -> Result<Pool> {
        let manager = ConnectionManager::memory();

        Ok(r2d2::Pool::new(manager)?)
    }

    pub fn get_one<C, T, P, F>(conn: &C, query: &str, p: P, f: F) -> Result<T>
    where
        P: Params,
        F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
        C: Deref<Target = Connection>,
        T: std::fmt::Debug,
    {
        let mut stmt = conn.prepare(query)?;
        let mut rows = stmt.query(p)?.mapped(f);

        match rows.next() {
            Some(value) => Ok(value?),
            None => anyhow::bail!(StorageError::EmptySet(query.to_string())),
        }
    }

    pub fn get_one_maybe<C, T, P, F>(conn: &C, query: &str, p: P, f: F) -> Result<Option<T>>
    where
        P: Params,
        F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
        C: Deref<Target = Connection>,
        T: std::fmt::Debug,
    {
        let mut stmt = conn.prepare(query)?;
        let mut rows = stmt.query(p)?.mapped(f);

        match rows.next() {
            Some(value) => Ok(Some(value?)),
            None => Ok(None),
        }
    }

    pub fn get<C, T, P, F>(conn: &C, query: &str, params: P, f: F) -> Result<Vec<T>>
    where
        P: rusqlite::Params,
        F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
        C: Deref<Target = Connection>,
        T: std::fmt::Debug,
    {
        let mut stmt = conn.prepare(query)?;
        let rows: rusqlite::Result<Vec<_>> = stmt.query_map(params, f)?.collect();

        Ok(rows?)
    }

    pub fn get_filtered<C, T, P, F>(conn: &C, query: &str, params: P, mut f: F) -> Result<Vec<T>>
    where
        P: rusqlite::Params,
        F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<Option<T>>,
        C: Deref<Target = Connection>,
        T: std::fmt::Debug,
    {
        let mut stmt = conn.prepare(query)?;
        let mut rows = stmt.query(params)?;
        let mut result = Vec::new();

        while let Some(row) = rows.next()? {
            if let Some(value) = f(&row)? {
                result.push(value);
            }
        }

        Ok(result)
    }


    pub fn explore<C, T, P, F>(conn: &C, query: &str, params: P, f: F) -> Result<()>
    where
        P: rusqlite::Params,
        F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
        C: Deref<Target = Connection>,
        T: std::fmt::Debug,
    {
        let mut stmt = conn.prepare(query)?;
        let rows = stmt.query_map(params, f)?;

        for row in rows {
            dbg!(row?);
        }

        Ok(())
    }

    pub fn table_list<C>(conn: &C) -> Result<()>
    where
        C: Deref<Target = Connection>,
    {
        Self::explore(conn, "pragma table_list", params![], |row| {
            let schema: String = row.get(0)?;
            let table: String = row.get(1)?;
            let kind: String = row.get(2)?;

            Ok((schema, table, kind))
        })?;

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("The query `{0}` returned an empty set.")]
    EmptySet(String),
}
