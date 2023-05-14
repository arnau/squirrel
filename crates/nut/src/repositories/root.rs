use crate::entities::storage::{params, Connection, Storage};
use crate::entities::{Result, Root};
use crate::repositories::Repository;
use std::ops::Deref;

pub struct RootRepository<'c, Conn: Deref<Target = Connection>>(pub &'c Conn);

impl<'c, Conn: Deref<Target = Connection>> Repository for RootRepository<'c, Conn> {}

impl<'c, Conn: Deref<Target = Connection>> RootRepository<'c, Conn> {
    pub fn get_all(&self) -> Result<Vec<Root>> {
        let query = r#"
        SELECT
            id,
            name,
            path,
            source_id
        FROM
            root
        "#;

        Storage::get(self.0, query, params![], Root::from_row)
    }

    pub fn get_by_id(&self, id: &str) -> Result<Option<Root>> {
        let query = r#"
        SELECT
            id,
            name,
            path,
            source_id
        FROM
            root
        WHERE
            id = ?
        "#;

        Storage::get_one_maybe(self.0, query, params![id], Root::from_row)
    }

}
