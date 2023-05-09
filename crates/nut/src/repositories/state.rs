use crate::entities::asset::AssetMetadata;
use crate::entities::state::{Asset, Folder};
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

    pub fn get_folders_with_count(&self, parent_id: &str) -> Result<Vec<(String, usize)>> {
        let query = r#"
        SELECT
            path,
            (
                SELECT
                    count(1)
                FROM
                    entry
                WHERE
                    entry.parent_id = main_entry.id
                AND
                    entry.kind = 'folder'
            ) AS count
        FROM
            entry AS main_entry
        WHERE
            main_entry.parent_id = ?
        AND
            main_entry.kind = 'folder'
        ORDER BY main_entry.path ASC
        "#;

        Storage::get(self.0, query, params![parent_id], |row| {
            let path: String = row.get(0)?;
            let count: usize = row.get(1)?;

            Ok((path, count))
        })
    }

    pub fn get_assets(&self, parent_id: &str) -> Result<Vec<Asset>> {
        let query = r#"
        SELECT
            asset.id,
            entry.path,
            asset.rating,
            asset.flag,
            asset.label,
            asset.format,
            asset.width,
            asset.height,
            asset.orientation,
            asset.master_id
        FROM
            asset
        INNER JOIN
            entry
            ON entry.id = asset.entry_id
        WHERE
            entry.parent_id = ?
        ORDER BY
            entry.path DESC
        "#;

        Storage::get(self.0, query, params![parent_id], |row| {
            let id: String = row.get(0)?;
            let path: String = row.get(1)?;
            let rating: Option<usize> = row.get(2)?;
            let flag: Option<bool> = row.get(3)?;
            let label: Option<String> = row.get(4)?;
            let format: String = row.get(5)?;
            let width: usize = row.get(6)?;
            let height: usize = row.get(7)?;
            let orientation: String = row.get(8)?;
            let master_id: Option<String> = row.get(9)?;
            let metadata = AssetMetadata {
                rating,
                flag,
                label,
                format,
                width,
                height,
                orientation,
            };

            Ok(Asset {
                id,
                path,
                master_id,
                metadata,
            })
        })
    }
}
