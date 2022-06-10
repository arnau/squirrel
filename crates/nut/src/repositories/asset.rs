use crate::entities::asset::{AssetMetadata, AssetRow};
use crate::entities::storage::{params, Connection, Storage};
use crate::entities::{Pyramid, Result, Source};
use crate::repositories::Repository;
use std::ops::Deref;

pub struct AssetRepository<'c, Conn: Deref<Target = Connection>>(pub &'c Conn);

impl<'c, Conn: Deref<Target = Connection>> Repository for AssetRepository<'c, Conn> {}

impl<'c, Conn: Deref<Target = Connection>> AssetRepository<'c, Conn> {
    pub fn get_for(&self, entry_id: &str) -> Result<AssetRow> {
        let query = r#"
        SELECT
            asset.id,
            asset.pyramid_filename,
            asset.rating,
            asset.flag,
            asset.label,
            asset.format,
            asset.width,
            asset.height,
            asset.orientation,
            source.path,
            source.name,
            source.version
        FROM
            asset
            INNER JOIN
                entry
                ON entry.id = asset.entry_id
            INNER JOIN
                source
                ON source.id = entry.source_id
        WHERE
            asset.entry_id = ?
        "#;

        Storage::get_one(self.0, query, params![entry_id], |row| {
            let id: String = row.get(0)?;
            let pyramid_filename: String = row.get(1)?;
            let rating: Option<usize> = row.get(2)?;
            let flag: Option<bool> = row.get(3)?;
            let label: Option<String> = row.get(4)?;
            let format: String = row.get(5)?;
            let width: usize = row.get(6)?;
            let height: usize = row.get(7)?;
            let orientation: String = row.get(8)?;
            let source_path: String = row.get(9)?;
            let source_name: String = row.get(10)?;
            let source_version: usize = row.get(11)?;
            let source = Source {
                name: source_name,
                path: source_path,
                version: source_version,
            };
            let previews_path = source
                .previews_path()
                .expect("Expected path to previews to exist.");
            let pyramid = Pyramid::new(previews_path, pyramid_filename);

            Ok(AssetRow {
                id,
                pyramid,
                metadata: AssetMetadata {
                    rating,
                    flag,
                    label,
                    format,
                    width,
                    height,
                    orientation,
                },
            })
        })
    }

    pub fn get_pyramid(&self, entry_id: &str) -> Result<Pyramid> {
        let query = r#"
        SELECT
            asset.pyramid_filename,
            source.path,
            source.name,
            source.version
        FROM
            asset
            INNER JOIN
                entry
                ON entry.id = asset.entry_id
            INNER JOIN
                source
                ON source.id = entry.source_id
        WHERE
            asset.entry_id = ?
        "#;

        Storage::get_one(self.0, query, params![entry_id], |row| {
            let pyramid_filename: String = row.get(0)?;
            let source_path: String = row.get(1)?;
            let source_name: String = row.get(2)?;
            let source_version: usize = row.get(3)?;
            let source = Source {
                name: source_name,
                path: source_path,
                version: source_version,
            };
            let previews_path = source
                .previews_path()
                .expect("Expected path to previews to exist.");
            let pyramid = Pyramid::new(previews_path, pyramid_filename);

            Ok(pyramid)
        })
    }
}