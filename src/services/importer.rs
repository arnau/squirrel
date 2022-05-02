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
/// - [x] Check version is known.
/// - [x] Find ID.
/// - [ ]
pub fn import(path: &str) -> Result<()> {
    let pool = Storage::file(path)?;
    let version = SourceRepository::version(pool.get()?)?;

    if !version.starts_with("11") {
        anyhow::bail!("The given catalogue has an unknown version: {}", version);
    }

    let id = SourceRepository::id(pool.get()?)?;
    let source = Source::try_new(id, version, path)?;

    dbg!(&source);
    dbg!(source.catalogue_path());
    dbg!(source.previews_path());

    Storage::explore(
        pool.get()?,
        "SELECT id_local, absolutePath FROM AgLibraryRootFolder",
        crate::entities::storage::params![],
        |row| {
            let id: u32 = row.get(0)?;
            let path: String = row.get(1)?;

            Ok((id, path))
        },
    )?;

    Ok(())
}
