use crate::entities::asset::{AssetId, Blob, BlobSize};
use crate::entities::entry::Kind;
use crate::entities::location::Location;
use crate::entities::state::{Asset, Folder};
use crate::entities::stem::{Stem, StemRow};
use crate::entities::storage::{Connection, Pool};
use crate::entities::{Event, Result, State};
use crate::repositories::{AssetRepository, EventRepository, StateRepository, StemRepository};
use anyhow::anyhow as ah;
use serde_json::json;
use std::ops::Deref;

/// Attempts to find the state for the given path.
///
/// Paths are expected to have a scope and a path:
///
/// - `cat:/2021/20210504_bla/bla_bla.jpg`
/// - `config:`
/// - `analytics:`
pub fn get_path(pool: &Pool, path: &str) -> Result<State> {
    let mut conn = pool.get()?;

    if path.starts_with('/') {
        let tx = conn.transaction()?;
        let state = get_catalogue_path(&tx, path)?;

        tx.commit()?;

        EventRepository::insert(
            &conn,
            &Event::new(
                "navigate",
                json!({
                    "path": path,
                }),
            ),
        )?;

        Ok(state)
    } else {
        Err(ah!("malformed path"))
    }
}

fn get_catalogue_path<C>(conn: &C, path: &str) -> Result<State>
where
    C: Deref<Target = Connection>,
{
    let location = get_location(conn, path)?;
    let state_repo = StateRepository(conn);

    let roots = state_repo.get_roots()?;
    let (folders, assets) = get_descendants(conn, location.ancestor())?;

    let state = if path == "/" {
        State::Catalogue {
            location,
            folders: roots.clone(),
            roots,
            assets,
        }
    } else {
        State::Catalogue {
            location,
            roots,
            folders,
            assets,
        }
    };

    dbg!(&state);

    Ok(state)
}

fn get_location<C>(conn: &C, path: &str) -> Result<Location>
where
    C: Deref<Target = Connection>,
{
    if let Some((path, asset_id)) = path.rsplit_once('#') {
        get_copy_location(conn, path, &asset_id.to_string())
    } else {
        let stem_rows = StemRepository::get(conn, path)?;

        let asset_repo = AssetRepository(conn);
        let mut stems = vec![];

        for StemRow { id, path, kind } in stem_rows {
            let stem = match kind {
                Kind::Folder => Stem::Folder { id, path },
                Kind::File => {
                    let asset_row = asset_repo.get_for(&id)?;
                    let metadata = asset_row.metadata;
                    let id = asset_row.id;
                    let master_id = asset_row.master_id;

                    Stem::Asset {
                        id,
                        path,
                        master_id,
                        metadata,
                    }
                }
            };

            stems.push(stem);
        }

        let location = Location::new(stems);

        Ok(location)
    }
}

fn get_copy_location<C>(conn: &C, path: &str, asset_id: &AssetId) -> Result<Location>
where
    C: Deref<Target = Connection>,
{
    let stem_rows = StemRepository::get(conn, path)?;
    let asset_repo = AssetRepository(conn);
    let mut stems = vec![];

    for StemRow { id, path, kind } in stem_rows {
        let stem = match kind {
            Kind::Folder => Stem::Folder { id, path },
            Kind::File => {
                let asset_row = asset_repo.get(asset_id)?;

                let metadata = asset_row.metadata;
                let id = asset_row.id;
                let master_id = asset_row.master_id;
                let path = format!("{}#{}", path, asset_id);

                Stem::Asset {
                    id,
                    path,
                    master_id,
                    metadata,
                }
            }
        };

        stems.push(stem);
    }

    let location = Location::new(stems);

    Ok(location)
}

fn get_descendants<C>(conn: &C, ancestor: Option<&Stem>) -> Result<(Vec<Folder>, Vec<Asset>)>
where
    C: Deref<Target = Connection>,
{
    let state_repo = StateRepository(conn);
    let data = if let Some(ancestor) = ancestor {
        (
            state_repo.get_folders(ancestor.id())?,
            state_repo.get_assets(ancestor.id())?,
        )
    } else {
        (vec![], vec![])
    };

    Ok(data)
}

/// Get the thumbnail for the given file.
///
/// This is not strictly navigatgion so might be best to move it to another service.
pub fn get_thumbnail(pool: &Pool, id: &AssetId) -> Result<Blob> {
    let conn = pool.get()?;
    let pyramid = AssetRepository(&conn).get_pyramid(id)?;
    let blob = pyramid.blob(BlobSize::Thumbnail)?;

    Ok(blob)
}

pub async fn get_async_thumbnail(pool: &Pool, id: &AssetId) -> Result<Blob> {
    let conn = pool.get()?;
    let pyramid = AssetRepository(&conn).get_pyramid(id)?;
    let blob = pyramid.async_blob(BlobSize::Thumbnail).await?;

    Ok(blob)
}

// TODO: Rename to Image to use Asset for what before was File.
// FIX: Incoming id should be the AssetId, but currenlty is the EntryId.
pub fn get_image(pool: &Pool, id: &AssetId) -> Result<Blob> {
    let conn = pool.get()?;
    let pyramid = AssetRepository(&conn).get_pyramid(id)?;
    let blob = pyramid.blob(BlobSize::Max)?;

    Ok(blob)
}
