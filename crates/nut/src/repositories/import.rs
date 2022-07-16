use crate::entities::import::Import;
use crate::entities::storage::{params, Connection, Storage};
use crate::entities::{Event, Pyramid, Result};
use crate::repositories::Repository;
use lazy_static::lazy_static;
use serde_json::json;
use std::include_str;
use std::ops::Deref;
use std::path::Path;

lazy_static! {
    static ref VIEWS_BOOTSTRAP: &'static str = include_str!("../storage/import.sql");
}

pub struct ImportRepository;

impl Repository for ImportRepository {}

impl ImportRepository {
    pub fn attach_catalogue<C>(conn: &C, source: &Import) -> Result<()>
    where
        C: Deref<Target = Connection>,
    {
        let query = format!(
            r#"
        PRAGMA foreign_keys = OFF;
        ATTACH DATABASE '{catalogue}' AS catalogue;
        ATTACH DATABASE '{previews}' AS previews;
        ATTACH DATABASE '{helper}' AS helper;
        "#,
            catalogue = source.catalogue_path()?.display(),
            previews = source.previews_path()?.display(),
            helper = source.helper_path()?.display()
        );

        conn.execute_batch(&query)?;
        conn.execute_batch(&VIEWS_BOOTSTRAP)?;

        Ok(())
    }

    pub fn detach_catalogue<C>(conn: &C) -> Result<()>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
        DROP VIEW import_root;
        DROP VIEW import_folder;
        DROP VIEW import_file;
        DROP VIEW import_asset;

        DETACH DATABASE catalogue;
        DETACH DATABASE previews;
        DETACH DATABASE helper;
        PRAGMA foreign_keys = ON;
        "#;

        conn.execute_batch(query)?;

        Ok(())
    }

    pub fn version<C>(conn: &C) -> Result<usize>
    where
        C: Deref<Target = Connection>,
    {
        // casting as text to minimise surprises. This table has a mix of types.
        // For example, `Adobe_entityIDCounter` autocasts as `Real`.
        let query = r#"
        SELECT
            cast(value as integer)
        FROM
            catalogue.Adobe_variablesTable
        WHERE
            name = 'Adobe_DBVersion'
        "#;
        Storage::get_one(conn, query, params![], |row| {
            let value: usize = row.get(0)?;

            Ok(value)
        })
    }

    pub fn id<C>(conn: &C) -> Result<String>
    where
        C: Deref<Target = Connection>,
    {
        // casting as text to minimise surprises. This table has a mix of types.
        // For example, `Adobe_entityIDCounter` autocasts as `Real`.
        let query = r#"
        SELECT
            cast(value as text)
        FROM
            catalogue.Adobe_variablesTable
        WHERE
            name = 'Adobe_storeProviderID'
        "#;
        Storage::get_one(conn, query, params![], |row| {
            let value: String = row.get(0)?;

            Ok(value)
        })
    }

    pub fn insert_source<C>(conn: &C, source: &Import) -> Result<()>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
            INSERT INTO source
                (id, name, path, version)
            VALUES
                (?, ?, ?, ?)
            "#;

        let id = Self::id(conn)?;
        let version = Self::version(conn)?;
        let mut stmt = conn.prepare(query)?;
        stmt.execute(params![
            &id,
            &source.name,
            &source.path.display().to_string(),
            &version,
        ])?;

        Ok(())
    }

    pub fn check_source_exists<C>(conn: &C) -> Result<bool>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
            SELECT
                count(id)
            FROM
                source
            WHERE
                id = ?
            "#;

        let id = Self::id(conn)?;

        Storage::get_one(conn, query, params![&id], |row| {
            let value: bool = row.get(0)?;

            Ok(value)
        })
    }

    pub fn copy_roots<C>(conn: &C) -> Result<()>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
            INSERT INTO root
            (id, name, path, source_id)
            SELECT
                *,
                ? AS source_id
            FROM
                import_root
            "#;

        let id = Self::id(conn)?;
        let mut stmt = conn.prepare(query)?;
        stmt.execute(params![&id])?;

        Ok(())
    }

    pub fn root_count<C>(conn: &C) -> Result<usize>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
            SELECT
                count(1)
            FROM
                import_root
            "#;

        Storage::get_one(conn, query, params![], |row| {
            let value: usize = row.get(0)?;

            Ok(value)
        })
    }

    pub fn copy_folders<C>(conn: &C) -> Result<()>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
            INSERT INTO entry
            (id, path, kind, parent_id, root_id, source_id)
            SELECT
                *,
                ? AS source_id
            FROM
                import_folder
            "#;

        let id = Self::id(conn)?;
        let mut stmt = conn.prepare(query)?;
        stmt.execute(params![&id])?;

        Ok(())
    }

    pub fn folder_count<C>(conn: &C) -> Result<usize>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
            SELECT
                count(id)
            FROM
                import_folder
            "#;

        Storage::get_one(conn, query, params![], |row| {
            let value: usize = row.get(0)?;

            Ok(value)
        })
    }

    pub fn copy_files<C>(conn: &C) -> Result<()>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
            INSERT INTO entry
            (id, path, kind, parent_id, root_id, source_id)
            SELECT
                *,
                ? AS source_id
            FROM
                import_file
            "#;

        let id = Self::id(conn)?;
        let mut stmt = conn.prepare(query)?;
        stmt.execute(params![&id])?;

        Ok(())
    }

    pub fn file_count<C>(conn: &C) -> Result<usize>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
            SELECT
                count(id)
            FROM
                import_file
            "#;

        Storage::get_one(conn, query, params![], |row| {
            let value: usize = row.get(0)?;

            Ok(value)
        })
    }

    pub fn copy_assets<C>(conn: &C) -> Result<()>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
            INSERT INTO asset
            (
                id,
                entry_id,
                rating,
                flag,
                label,
                format,
                width,
                height,
                orientation,
                master_id,
                pyramid_uuid,
                pyramid_digest,
                modification_time
            )
            SELECT
                *
            FROM
                import_asset
            "#;

        let mut stmt = conn.prepare(query)?;
        stmt.execute(params![])?;

        Ok(())
    }

    pub fn asset_count<C>(conn: &C) -> Result<usize>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
            SELECT
                count(id)
            FROM
                import_asset
            "#;

        Storage::get_one(conn, query, params![], |row| {
            let value: usize = row.get(0)?;

            Ok(value)
        })
    }

    pub fn check_broken_pyramids<C>(conn: &C, previews_path: &Path) -> Result<Vec<Event>>
    where
        C: Deref<Target = Connection>,
    {
        Storage::get_filtered(
            conn,
            r#"
        SELECT
            entry.id,
            entry.path,
            asset.pyramid_filename,
            asset.orientation
        FROM
            asset
        INNER JOIN
            entry
            ON entry.id = asset.entry_id
        "#,
            params![],
            |row| {
                let entry_id: String = row.get(0)?;
                let path: String = row.get(1)?;
                let pyramid_filename: String = row.get(2)?;
                let orientation: String = row.get(3)?;
                let pyramid_fullpath =
                    Pyramid::new(previews_path.to_path_buf(), &pyramid_filename, &orientation)
                        .absolute_path();

                if pyramid_fullpath.exists() {
                    Ok(None)
                } else {
                    let event = Event::new(
                        "import:missing_pyramid",
                        json!({
                            "entry_id": entry_id,
                            "entry_path": path,
                            "pyramid_path": pyramid_fullpath,
                        }),
                    );

                    Ok(Some(event))
                }
            },
        )
    }
}
