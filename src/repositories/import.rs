use crate::entities::import::Import;
use crate::entities::storage::{params, Connection, Storage};
use crate::entities::{Event, Result};
use crate::repositories::Repository;
use serde_json::json;
use std::ops::Deref;
use std::path::Path;

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

        Ok(())
    }

    pub fn detach_catalogue<C>(conn: &C) -> Result<()>
    where
        C: Deref<Target = Connection>,
    {
        let query = format!(
            r#"
        DETACH DATABASE catalogue;
        DETACH DATABASE previews;
        DETACH DATABASE helper;
        PRAGMA foreign_keys = ON;
        "#,
        );

        conn.execute_batch(&query)?;

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
        let mut stmt = conn.prepare(&query)?;
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
                id_global AS id,
                name,
                absolutePath AS path,
                ? AS source_id
            FROM
                catalogue.AgLibraryRootFolder
            WHERE
                relativePathFromCatalog IS NOT NULL
            "#;

        let id = Self::id(conn)?;
        let mut stmt = conn.prepare(&query)?;
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
                catalogue.AgLibraryRootFolder
            WHERE
                relativePathFromCatalog IS NOT NULL
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
                folder.id_global AS id,
                '/' || root_folder.name || '/' || folder.pathFromRoot AS path,
                'folder' AS kind,
                (
                    CASE
                        WHEN parent_path(folder.pathFromRoot) IS NOT NULL THEN
                            (
                                SELECT
                                    f.id_global
                                FROM
                                    AgLibraryFolder AS f
                                WHERE
                                    f.pathFromRoot = parent_path(folder.pathFromRoot)
                                LIMIT 1
                            )
                        ELSE
                            NULL
                    END
                ) AS parent_id,
                root_folder.id_global AS root_id,
                ? AS source_id
            FROM
                catalogue.AgLibraryFolder AS folder
                JOIN
                    catalogue.AgLibraryRootFolder AS root_folder
                    ON folder.rootFolder = root_folder.id_local
            WHERE
                root_folder.relativePathFromCatalog IS NOT NULL
            "#;

        let id = Self::id(conn)?;
        let mut stmt = conn.prepare(&query)?;
        stmt.execute(params![&id])?;

        Ok(())
    }

    pub fn folder_count<C>(conn: &C) -> Result<usize>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
            SELECT
                count(folder.id_global)
            FROM
                catalogue.AgLibraryFolder AS folder
                JOIN
                    catalogue.AgLibraryRootFolder AS root_folder
                    ON folder.rootFolder = root_folder.id_local
            WHERE
                root_folder.relativePathFromCatalog IS NOT NULL
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
                file.id_global AS id,
                '/' || root_folder.name || '/' || folder.pathFromRoot || file.idx_filename AS path,
                'file' AS kind,
                folder.id_global AS parent_id,
                root_folder.id_global AS root_id,
                ? AS source_id
            FROM
                catalogue.AgLibraryFile AS file
                JOIN
                    catalogue.AgLibraryFolder AS folder
                    ON file.folder = folder.id_local
                JOIN
                    catalogue.AgLibraryRootFolder AS root_folder
                    ON root_folder.id_local = folder.rootFolder
            WHERE
                root_folder.relativePathFromCatalog IS NOT NULL
            "#;

        let id = Self::id(conn)?;
        let mut stmt = conn.prepare(&query)?;
        stmt.execute(params![&id])?;

        Ok(())
    }

    pub fn file_count<C>(conn: &C) -> Result<usize>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
            SELECT
                count(file.id_global)
            FROM
                catalogue.AgLibraryFile AS file
                JOIN
                    catalogue.AgLibraryFolder AS folder
                    ON file.folder = folder.id_local
                JOIN
                    catalogue.AgLibraryRootFolder AS root_folder
                    ON root_folder.id_local = folder.rootFolder
            WHERE
                root_folder.relativePathFromCatalog IS NOT NULL
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
                pyramid_uuid,
                pyramid_digest,
                modification_time
            )
            SELECT
                image.id_global AS id,
                file.id_global AS entry_id,

                COALESCE(image.rating, 0) AS rating,
                image.pick AS flag,
                image.colorLabels AS label,
                image.fileFormat AS format,
                image.fileWidth AS width,
                image.fileHeight AS height,
                image.orientation AS orientation,

                cache.uuid AS pyramid_uuid,
                cache.digest AS pyramid_digest,

                strftime('%Y-%m-%dT%H:%M:%SZ', 
                    CASE
                      WHEN file.modTime IS NULL THEN
                        63113817600
                      ELSE
                        file.modTime + 978307200
                    END, 'unixepoch')
                AS modification_stamp
            FROM
                catalogue.Adobe_images AS image
            JOIN
                catalogue.AgLibraryFile AS file
                ON file.id_local = image.rootFile
            JOIN
                catalogue.AgLibraryFolder AS folder
                ON file.folder = folder.id_local
            JOIN
                catalogue.AgLibraryRootFolder AS root_folder
                ON folder.rootFolder = root_folder.id_local
            JOIN
                previews.ImageCacheEntry AS cache 
                ON cache.imageId = image.id_local
            WHERE
                root_folder.relativePathFromCatalog IS NOT NULL
            "#;

        let mut stmt = conn.prepare(&query)?;
        stmt.execute(params![])?;

        Ok(())
    }

    pub fn asset_count<C>(conn: &C) -> Result<usize>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
            SELECT
                count(image.id_global)
            FROM
                catalogue.Adobe_images AS image
            JOIN
                catalogue.AgLibraryFile AS file
                ON file.id_local = image.rootFile
            JOIN
                catalogue.AgLibraryFolder AS folder
                ON file.folder = folder.id_local
            JOIN
                catalogue.AgLibraryRootFolder AS root_folder
                ON folder.rootFolder = root_folder.id_local
            JOIN
                previews.ImageCacheEntry AS cache 
                ON cache.imageId = image.id_local
            WHERE
                root_folder.relativePathFromCatalog IS NOT NULL
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
            asset.pyramid_filename
        FROM
            asset
        JOIN
            entry
            ON entry.id = asset.entry_id
        "#,
            params![],
            |row| {
                let entry_id: String = row.get(0)?;
                let path: String = row.get(1)?;
                let pyramid_filename: String = row.get(2)?;

                let nibble = pyramid_filename.get(0..1).unwrap();
                let two_bytes = pyramid_filename.get(0..4).unwrap();
                let pyramid_path = format!("{}/{}/{}", nibble, two_bytes, pyramid_filename);
                let pyramid_fullpath = previews_path.join(&pyramid_path);

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
