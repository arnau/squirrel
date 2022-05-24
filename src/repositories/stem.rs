use crate::entities::entry::Kind;
use crate::entities::stem::StemRow;
use crate::entities::storage::{params, Connection, Storage};
use crate::entities::Result;
use crate::repositories::Repository;
use std::ops::Deref;

pub struct StemRepository;

impl Repository for StemRepository {}

impl StemRepository {
    pub fn get<C>(conn: &C, path: &str) -> Result<Vec<StemRow>>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
        WITH RECURSIVE stem(id, path, parent_id, kind) AS (
        SELECT
            entry.id,
            entry.path,
            entry.parent_id,
            entry.kind
        FROM
            entry
        WHERE
            path = ?
        UNION
        SELECT
            entry.id,
            entry.path,
            entry.parent_id,
            entry.kind
        FROM
            entry
        INNER JOIN
            stem
            ON
                stem.parent_id = entry.id
        )
        SELECT
            id,
            path,
            kind
        FROM
            stem
        ORDER BY
            path ASC
        "#;

        Storage::get(conn, query, params![path], |row| {
            let id: String = row.get(0)?;
            let path: String = row.get(1)?;
            let kind: Kind = row.get(2)?;

            Ok(StemRow { id, path, kind })
        })
    }
}
