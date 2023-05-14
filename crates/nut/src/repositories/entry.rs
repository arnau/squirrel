use crate::entities::storage::{params, Connection, Storage};
use crate::entities::{Entry, Result};
use crate::repositories::Repository;
use std::ops::Deref;

pub struct EntryRepository;

impl Repository for EntryRepository {}

impl EntryRepository {
    pub fn find_by_path<C>(conn: &C, path: &str) -> Result<Option<Entry>>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
        SELECT
            id,
            path,
            kind,
            parent_id,
            root_id,
            source_id
        FROM
            entry
        WHERE
            path = ?
        "#;

        Storage::get_one_maybe(conn, query, params![path], Entry::from_row)
    }

    pub fn find_by_id<C>(conn: &C, id: &str) -> Result<Option<Entry>>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
        SELECT
            id,
            path,
            kind,
            parent_id,
            root_id,
            source_id
        FROM
            entry
        WHERE
            id = ?
        "#;

        Storage::get_one_maybe(conn, query, params![id], Entry::from_row)
    }

    // TODO: Should this be here or in a different repo?
    pub fn find_folder_count<C>(conn: &C, id: &str) -> Result<usize>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
        SELECT
            count(1)
        FROM
            entry AS asset
        WHERE
            parent_id = ?
        AND
            kind = 'folder'
        "#;

        Storage::get_one(conn, query, params![id], |row| {
            let count: usize = row.get(0)?;

            Ok(count)
        })
    }

    // TODO: Should this be here or in a different repo?
    pub fn find_asset_count<C>(conn: &C, id: &str) -> Result<usize>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
        SELECT
            count(1)
        FROM
            entry AS asset
        WHERE
            parent_id = ?
        AND
            kind = 'file'
        "#;

        Storage::get_one(conn, query, params![id], |row| {
            let count: usize = row.get(0)?;

            Ok(count)
        })
    }
}
