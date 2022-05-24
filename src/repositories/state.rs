use crate::entities::state::{File, Folder};
use crate::entities::storage::{params, Connection, Storage};
use crate::entities::Result;
use crate::repositories::Repository;
use std::ops::Deref;

pub struct StateRepository<'c, Conn: Deref<Target = Connection>>(pub &'c Conn);

impl<'c, Conn: Deref<Target = Connection>> Repository for StateRepository<'c, Conn> {}

impl<'c, Conn: Deref<Target = Connection>> StateRepository<'c, Conn> {
    pub fn get_roots(&self) -> Result<Vec<Folder>> {
        let query = r#"
        SELECT
            id,
            path
        FROM
            entry
        WHERE
            parent_id IS NULL
        "#;

        Storage::get(self.0, query, params![], |row| {
            let id: String = row.get(0)?;
            let path: String = row.get(1)?;

            Ok(Folder { id, path })
        })
    }

    pub fn get_folders(&self, parent_id: &str) -> Result<Vec<Folder>> {
        let query = r#"
        SELECT
            id,
            path
        FROM
            entry
        WHERE
            parent_id = ?
        AND
            kind = 'folder'
        "#;

        Storage::get(self.0, query, params![parent_id], |row| {
            let id: String = row.get(0)?;
            let path: String = row.get(1)?;

            Ok(Folder { id, path })
        })
    }

    pub fn get_files(&self, parent_id: &str) -> Result<Vec<File>> {
        let query = r#"
        SELECT
            id,
            path
        FROM
            entry
        WHERE
            parent_id = ?
        AND
            kind = 'file'
        ORDER BY
            path DESC
        "#;

        Storage::get(self.0, query, params![parent_id], |row| {
            let id: String = row.get(0)?;
            let path: String = row.get(1)?;

            Ok(File { id, path })
        })
    }
}
