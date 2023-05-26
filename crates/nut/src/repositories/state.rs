use crate::entities::asset::{AssetId, AssetMetadata};
use crate::entities::state::{Asset, AssetCursor, AssetPath, FolderEntry, FolderId, FolderPath};
use crate::entities::storage::{params, Connection, Storage};
use crate::entities::Result;
use crate::repositories::Repository;
use std::ops::Deref;

pub struct StateRepository<'c, Conn: Deref<Target = Connection>>(pub &'c Conn);

impl<'c, Conn: Deref<Target = Connection>> Repository for StateRepository<'c, Conn> {}

impl<'c, Conn: Deref<Target = Connection>> StateRepository<'c, Conn> {
    pub fn get_folders(&self, parent_id: &FolderId) -> Result<Vec<FolderEntry>> {
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
            let id: FolderId = row.get(0)?;
            let path: FolderPath = row.get(1)?;
            let counter: usize = row.get(2)?;

            Ok(FolderEntry { id, path, counter })
        })
    }

    pub fn get_folder_trail(&self, id: &FolderId) -> Result<Vec<FolderId>> {
        let query = r#"
            WITH RECURSIVE mark(idx, id, parent_id) AS (
            SELECT
                1 AS idx,
                entry.id,
                entry.parent_id
            FROM
                entry
            WHERE
                id = ?1
            UNION ALL
            SELECT
                mark.idx + 1,
                entry.id,
                entry.parent_id
            FROM
                entry
            INNER JOIN
                mark
                ON
                    mark.parent_id = entry.id
            )
            SELECT
                mark.id
            FROM
                mark
            WHERE
                mark.idx > 1
            ORDER BY
                mark.idx DESC
        "#;

        Storage::get(self.0, query, params![id], |row| {
            let id: FolderId = row.get(0)?;

            Ok(id)
        })
    }

    pub fn get_asset_trail(&self, id: &AssetId) -> Result<Vec<FolderId>> {
        let query = r#"
            WITH RECURSIVE mark(idx, kind, id, parent_id) AS (
                SELECT
                    1 AS idx,
                    'asset' AS kind,
                    asset.id,
                    asset.entry_id
                FROM
                    asset
                INNER JOIN
                    entry ON entry.id = asset.entry_id
                WHERE
                    asset.id = ?1

            UNION ALL

                SELECT
                    mark.idx + 1,
                    entry.kind,
                    entry.id,
                    entry.parent_id
                FROM
                    entry
                INNER JOIN
                    mark
                    ON
                        mark.parent_id = entry.id
            )
            SELECT
                mark.id
            FROM
                mark
            WHERE
                mark.kind == 'folder'
            ORDER BY
                mark.idx DESC
        "#;

        Storage::get(self.0, query, params![id], |row| {
            let id: FolderId = row.get(0)?;

            Ok(id)
        })
    }

    pub fn get_folder_path(&self, id: &FolderId) -> Result<FolderPath> {
        let query = r#"
            SELECT
                entry.path AS path
            FROM
                entry
            WHERE
                entry.id = ?1
        "#;

        Storage::get_one(self.0, query, params![id], |row| {
            let path: FolderPath = row.get(0)?;

            Ok(path)
        })
    }

    pub fn get_asset_path(&self, id: &AssetId) -> Result<AssetPath> {
        let query = r#"
            SELECT
                iif(asset.master_id is null, entry.path, entry.path || '#' || asset.id) AS path
            FROM
                asset
            INNER JOIN
                entry ON entry.id = asset.entry_id
            WHERE
                asset.id = ?1
        "#;

        Storage::get_one(self.0, query, params![id], |row| {
            let path: AssetPath = row.get(0)?;

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
        parent_id: &FolderId,
        cursor: Option<AssetCursor>,
    ) -> Result<(Option<AssetCursor>, Vec<Asset>)> {
        let cursor_filter = if let Some(cursor) = cursor {
            format!("AND asset_path > '{cursor}'")
        } else {
            format!("")
        };
        let query = format!(
            r#"
        SELECT
            asset.id,
            iif(master_id is null, entry.path, entry.path || '#' || asset.id) AS asset_path,
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
        "#
        );

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
