use crate::entities::storage::{params, Connection, Storage};
use crate::entities::Result;
use crate::repositories::Repository;
use crate::Version;
use std::ops::Deref;

pub struct CatalogueRepository;

impl Repository for CatalogueRepository {}

impl CatalogueRepository {
    pub fn check_exists<C>(conn: &C) -> Result<bool>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
        SELECT
            count(1)
        FROM
            pragma_table_list
        "#;

        Storage::get_one(conn, query, params![], |row| {
            let value: usize = row.get(0)?;

            // Schema has 6 tables. SQLite adds sqlite_schema and sqlite_temp_schema.
            Ok(value == 8)
        })
    }

    pub fn version<C>(conn: &C) -> Result<Version>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
        SELECT
            cast(value as text)
        FROM
            catalogue_metadata
        WHERE
            key = 'version'
        "#;

        Storage::get_one(conn, query, params![], |row| {
            let value: String = row.get(0)?;
            let triple: Version =
                serde_json::from_str(&value).expect("can't read catalogue version");

            Ok(triple)
        })
    }

    pub fn insert_version<C>(conn: &C, version: Version) -> Result<()>
    where
        C: Deref<Target = Connection>,
    {
        let value = serde_json::to_string(&version)?;

        Self::insert_pair(conn, "version", &value)
    }

    pub fn insert_pair<C>(conn: &C, key: &str, value: &str) -> Result<()>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
            INSERT INTO catalogue_metadata
                (key, value)
            VALUES
                (?, ?)
            "#;

        let mut stmt = conn.prepare(&query)?;
        stmt.execute(params![key, value,])?;

        Ok(())
    }
}
