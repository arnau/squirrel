use crate::entities::asset::{AssetMetadata, AssetId};
use crate::entities::entry::{EntryId, EntryPath};
use crate::entities::state::{Asset, AssetCursor, FolderEntry, AssetPath};
use crate::entities::storage::{params, Connection, Storage};
use crate::entities::Result;
use crate::repositories::Repository;
use std::ops::Deref;

pub struct StateRepository<'c, Conn: Deref<Target = Connection>>(pub &'c Conn);

impl<'c, Conn: Deref<Target = Connection>> Repository for StateRepository<'c, Conn> {}

impl<'c, Conn: Deref<Target = Connection>> StateRepository<'c, Conn> {
    pub fn get_folders(&self, parent_id: &str) -> Result<Vec<FolderEntry>> {
        let query = r#"
        SELECT
            id,
            path,
            (
                SELECT
                    count(1)
                FROM
                    entry AS child_entry
                WHERE
                    child_entry.parent_id = entry.id
                AND
                    child_entry.kind = 'folder'

            ) AS counter
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
            let counter: usize = row.get(2)?;

            Ok(FolderEntry { id, path, counter })
        })
    }

    pub fn get_trail(&self, id: &str) -> Result<Vec<EntryId>> {
        let query = r#"
        WITH RECURSIVE stem(id, path, parent_id) AS (
        SELECT
            entry.id,
            entry.path,
            entry.parent_id
        FROM
            entry
        WHERE
            id = ?
        UNION
        SELECT
            entry.id,
            entry.path,
            entry.parent_id
        FROM
            entry
        INNER JOIN
            stem
            ON
                stem.parent_id = entry.id
        )
        SELECT
            id
        FROM
            stem
        WHERE
            stem.id <> ?
        ORDER BY
            path ASC
        "#;

        Storage::get(self.0, query, params![id], |row| {
            let id: String = row.get(0)?;

            Ok(id)
        })
    }

    pub fn get_location_path(&self, id: &EntryId) -> Result<EntryId> {
        let query = r#"
        SELECT
            entry.path
        FROM
            entry
        WHERE
            id = ?
        "#;

        Storage::get_one(self.0, query, params![id], |row| {
            let path: EntryPath = row.get(0)?;

            Ok(path)
        })
    }


    pub fn get_roots(&self) -> Result<Vec<FolderEntry>> {
        let query = r#"
        SELECT
            id,
            path,
            (
                SELECT
                    count(1)
                FROM
                    entry AS child_entry
                WHERE
                    child_entry.parent_id = entry.id
                AND
                    child_entry.kind = 'folder'

            ) AS counter
        FROM
            entry
        WHERE
            parent_id IS NULL
        "#;

        Storage::get(self.0, query, params![], |row| {
            let id: String = row.get(0)?;
            let path: String = row.get(1)?;
            let counter: usize = row.get(2)?;

            Ok(FolderEntry { id, path, counter })
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

    pub fn get_asset_page(
        &self,
        parent_id: &EntryId,
        cursor: Option<AssetCursor>,
    ) -> Result<(Option<AssetCursor>, Vec<Asset>)> {
        let cursor_filter = if let Some(cursor) = cursor {
            format!("AND asset_path > '{cursor}'")
        } else {
            format!("")
        };
        let query = format!(r#"
        SELECT
            asset.id,
            ifnull(entry.path || '+' || asset.master_id, entry.path) AS asset_path,
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
        {cursor_filter}
        ORDER BY
            asset_path ASC
        LIMIT
            100
        "#);

        let assets = Storage::get(self.0, &query, params![parent_id], |row| {
            let id: AssetId = row.get(0)?;
            let path: AssetPath = row.get(1)?;
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
        })?;

        Ok((assets.last().map(|asset| asset.path.clone()), assets))
    }
}
