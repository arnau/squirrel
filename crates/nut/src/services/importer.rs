use crate::entities::import::{Import, ImportError};
use crate::entities::storage::Pool;
use crate::entities::{Event, Result};
use crate::repositories::{EventRepository, ImportRepository};
use anyhow::anyhow as ah;
use serde_json::json;

/// Attempts to import the information contained in the given Lightroom catalogue.
///
/// The given path must be a `.lrcat` catalogue. It assumes a layout structure like:
///
/// ```text
/// foo/
///   foo.lrcat
///   foo Helper.lrdata/
///     helper.db
///   foo Previews.lrdata/
///     previews.db
///     root-pixels.db
/// ```
#[allow(clippy::inconsistent_digit_grouping)]
pub fn import(pool: &Pool, path: &str) -> Result<()> {
    let mut conn = pool.get()?;

    EventRepository::insert(&conn, &Event::new("import:start", json!({ "path": path })))?;

    let tx = conn.transaction()?;
    let source = Import::try_new(path)?;

    ImportRepository::attach_catalogue(&tx, &source)?;

    // TODO: Handle source updates. E.g. ratings updated.
    if ImportRepository::check_source_exists(&tx)? {
        return Err(ah!(ImportError::SourceExists));
    }

    let version = ImportRepository::version(&tx)?;

    if !(11_00_00_0..12_00_00_0).contains(&version) {
        return Err(ah!(ImportError::UnknownVersion(version)));
    }

    ImportRepository::insert_source(&tx, &source)?;
    ImportRepository::copy_roots(&tx)?;
    ImportRepository::copy_folders(&tx)?;
    ImportRepository::copy_files(&tx)?;
    ImportRepository::copy_assets(&tx)?;

    let mut previews_path = source.previews_path()?;
    previews_path.pop();

    let broken_pyramids = ImportRepository::check_broken_pyramids(&tx, &previews_path)?;

    for event in &broken_pyramids {
        EventRepository::insert(&tx, event)?;
    }

    let report = json!({
        "root_count": ImportRepository::root_count(&tx)?,
        "folder_count": ImportRepository::folder_count(&tx)?,
        "file_count": ImportRepository::file_count(&tx)?,
        "asset_count": ImportRepository::asset_count(&tx)?,
        "broken_pyramid_count": broken_pyramids.len(),
        "path": path
    });

    tx.commit()?;

    ImportRepository::detach_catalogue(&conn)?;
    EventRepository::insert(&conn, &Event::new("import:end", report))?;

    Ok(())
}
