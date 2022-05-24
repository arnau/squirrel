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
}
