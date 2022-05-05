//! This module contains the repository for the source Lightroom catalogue.
//!
//! All actions are expected to be resolved against a valid Lightroom schema.
//!
//! This is NOT about the `Source` entity. TODO: Update this when the repository is known.

use thiserror::private::PathAsDisplay;

use crate::entities::storage::{params, Connection, Storage};
use crate::entities::{Result, Source};

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

    pub fn id(conn: Connection) -> Result<String> {
        // casting as text to minimise surprises. This table has a mix of types.
        // For example, `Adobe_entityIDCounter` autocasts as `Real`.
        let query = r#"
        SELECT
            cast(value as text)
        FROM
            Adobe_variablesTable
        WHERE
            name = 'Adobe_storeProviderID'
        "#;
        Storage::get_one(conn, query, params![], |row| {
            let value: String = row.get(0)?;

            Ok(value)
        })
    }

    pub fn attach_aux(conn: Connection, source: &Source) -> Result<()> {
        let query = format!(
            r#"
            ATTACH DATABASE '{previews}' AS previews;
            ATTACH DATABASE '{helper}' AS helper;
            "#,
            previews = source.previews_path().as_display(),
            helper = source.helper_path().as_display()
        );

        let mut stmt = conn.prepare(&query)?;
        stmt.execute(params![])?;

        Ok(())
    }

    pub fn roots(conn: Connection) -> Result<()> {
        let query = r#"
            SELECT
                id_global,
                name,
                absolutePath
            FROM
                AgLibraryRootFolder
            WHERE
                relativePathFromCatalog IS NOT NULL
            "#;
        Ok(())
    }

    pub fn folders(conn: Connection) -> Result<()> {
        let query = r#"
            SELECT
                folder.id_global AS id,
                '/' || root.name || '/' || folder.pathFromRoot AS path,
                root.id_global AS root_id
            FROM
                AgLibraryFolder AS folder
            JOIN
                AgLibraryRootFolder AS root
            ON
                folder.rootFolder = root.id_local
            WHERE
                root.relativePathFromCatalog IS NOT NULL
            "#;
        Ok(())
    }

    pub fn files(conn: Connection) -> Result<()> {
        let query = r#"
            SELECT
                file.id_global AS id,
                '/' || root.name || '/' || folder.pathFromRoot || file.originalFilename AS path,
                folder.id_global AS parent_id,
                root.id_global AS root_id
            FROM
                AgLibraryFile AS file
            JOIN
                AgLibraryFolder AS folder
            ON
                file.folder = folder.id_local
            JOIN
                AgLibraryRootFolder AS root
            ON
                folder.rootFolder = root.id_local
            WHERE
                root.relativePathFromCatalog IS NOT NULL
            "#;
        Ok(())
    }

    pub fn assets(conn: Connection) -> Result<()> {
        let query = r#"
            SELECT
                image.id_global AS id,
                file.id_global AS entry_id,

                COALESCE(image.rating, 0) AS rating,
                file.extension AS extension,
                image.fileFormat AS format,
                image.colorLabels AS label,
                image.fileWidth AS width,
                image.fileHeight AS height,
                image.orientation AS orientation,


                strftime('%Y-%m-%dT%H:%M:%SZ', 
                    CASE
                      WHEN file.modTime IS NULL THEN
                        63113817600
                      ELSE
                        file.modTime + 978307200
                    END, 'unixepoch')
                AS modification_stamp
            FROM
                Adobe_images AS image
            JOIN
                AgLibraryFile AS file
            ON 
                file.id_local = image.rootFile
            JOIN
                AgLibraryFolder AS folder
            ON
                file.folder = folder.id_local
            JOIN
                AgLibraryRootFolder AS root
            ON
                folder.rootFolder = root.id_local
            WHERE
                root.relativePathFromCatalog IS NOT NULL
            "#;
        Ok(())
    }
}
