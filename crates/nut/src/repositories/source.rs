use crate::entities::storage::{params, Connection, Storage};
use crate::entities::{Result, Source};
use crate::repositories::Repository;
use std::ops::Deref;

// TODO: Normalise Source (entity and state)
pub struct SourceRepository<'c, Conn: Deref<Target = Connection>>(pub &'c Conn);

impl<'c, Conn: Deref<Target = Connection>> Repository for SourceRepository<'c, Conn> {}

impl<'c, Conn: Deref<Target = Connection>> SourceRepository<'c, Conn> {
    pub fn get_all(&self) -> Result<Vec<Source>> {
        let query = r#"
        SELECT
            name,
            path,
            version
        FROM
            source
        "#;

        Storage::get(self.0, query, params![], |row| {
            let name: String = row.get(0)?;
            let path: String = row.get(1)?;
            let version: usize = row.get(2)?;

            let source = Source {
                name,
                path,
                version,
            };

            Ok(source)
        })
    }

    pub fn get_by_id(&self, id: &str) -> Result<Option<Source>> {
        let query = r#"
        SELECT
            name,
            path,
            version
        FROM
            source
        WHERE
            id = ?
        "#;

        Storage::get_one_maybe(self.0, query, params![id], |row| {
            let name: String = row.get(0)?;
            let path: String = row.get(1)?;
            let version: usize = row.get(2)?;

            let source = Source {
                name,
                path,
                version,
            };

            Ok(source)
        })
    }
}
