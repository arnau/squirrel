use anyhow::Result;

use crate::entities::storage::{params, Connection, Storage};

trait Repository {}

/// Abstracts the interaction with Storage regarding the Source.
#[derive(Debug, Clone)]
pub struct SourceRepository;

impl Repository for SourceRepository {}

impl SourceRepository {
    pub fn version(conn: Connection) -> Result<String> {
        // casting as text to minimise surprises. This table has a mix of types.
        // For example, `Adobe_entityIDCounter` autocasts as `Real`.
        let query = r#"
        SELECT
            cast(value as text)
        FROM
            Adobe_variablesTable
        WHERE
            name = 'Adobe_DBVersion'
        "#;
        Storage::get_one(conn, query, params![], |row| {
            let value: String = row.get(0)?;

            Ok(value)
        })
    }
}
