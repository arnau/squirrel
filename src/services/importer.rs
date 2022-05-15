use crate::entities::import::{Import, ImportError};
use crate::entities::Event;
use crate::entities::{storage::Pool, Result};
use crate::repositories::import::ImportRepository;
use crate::repositories::EventRepository;
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
pub fn import(pool: &Pool, path: &str) -> Result<()> {
    let mut conn = pool.get()?;

    EventRepository::insert(&conn, &Event::new("import:start", json!({ "path": path })))?;

    let tx = conn.transaction()?;
    let source = Import::try_new(path)?;

    ImportRepository::attach_catalogue(&tx, &source)?;

    let version = ImportRepository::version(&tx)?;

    if !(version >= 11_00_00_0 && version < 12_00_00_0) {
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

    for event in broken_pyramids {
        EventRepository::insert(&tx, &event)?;
    }

    tx.commit()?;

    ImportRepository::detach_catalogue(&conn)?;
    EventRepository::insert(&conn, &Event::new("import:end", json!({ "path": path })))?;

    let events = EventRepository::head(&conn, 4)?;

    dbg!(events);

    Ok(())
}
