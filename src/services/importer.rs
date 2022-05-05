use crate::entities::storage::params;
use crate::entities::{Result, Source, Storage};
use crate::repositories::source::SourceRepository;

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
/// `
/// Plan:
///
/// - [ ] Connect to Squirrel catalogue.
/// - [x] Check version is known.
/// - [x] Find ID.
/// - [x] Attach aux databases.
/// - [ ] Check source id exists. Insert if not, abort otheriwse.
/// - [ ] Insert root folders into table root. AgLibraryRootFolder.
/// - [ ] Insert folders into table entry. AgLibraryFolder. Reconstruct full paths.
/// - [ ] Insert files into table entry. AgLibraryFile. Reconstruct full paths.
/// - [ ] Insert assets into table asset. Mix of Adobe_Images, previews.ImageCacheEntry, etc.
pub fn import(path: &str) -> Result<()> {
    let pool = Storage::file(path)?;
    let version = SourceRepository::version(pool.get()?)?;

    if !version.starts_with("11") {
        anyhow::bail!("The given catalogue has an unknown version: {}", version);
    }

    let id = SourceRepository::id(pool.get()?)?;
    let source = Source::try_new(id, version, path)?;

    SourceRepository::attach_aux(pool.get()?, &source)?;

    dbg!(&source);

    Storage::explore(pool.get()?, "pragma table_list", params![], |row| {
        let schema: String = row.get(0)?;
        let table: String = row.get(1)?;
        let kind: String = row.get(2)?;

        Ok((schema, table, kind))
    })?;

    Storage::explore(
        pool.get()?,
        "SELECT id_local, absolutePath FROM AgLibraryRootFolder",
        params![],
        |row| {
            let id: u32 = row.get(0)?;
            let path: String = row.get(1)?;

            Ok((id, path))
        },
    )?;

    Ok(())
}
