use crate::entities::asset::{AssetId, Blob, BlobSize};
use crate::entities::entry::Kind;
use crate::entities::location::Location;
use crate::entities::state::{Asset, Folder, Tree};
use crate::entities::stem::{Stem, StemRow};
use crate::entities::storage::{Connection, Pool};
use crate::entities::{Event, Result, State};
use crate::repositories::{
    AssetRepository, EntryRepository, EventRepository, StateRepository, StemRepository,
};
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

/// Attempts to get the information for the given route.
pub fn get_route(pool: &Pool, route: &str) -> Result<State> {
    let mut conn = pool.get()?;

    if route.starts_with('/') && route.ends_with('/') {
        let tx = conn.transaction()?;
        // let state = get_catalogue_route(&tx, route)?;
        let state = get_catalogue_path(&tx, route)?;

        tx.commit()?;

        EventRepository::insert(
            &conn,
            &Event::new(
                "navigate:route",
                json!({
                    "path": route,
                }),
            ),
        )?;

        Ok(state)
    } else {
        Err(ah!("malformed path"))
    }
}

/// Returns the set of available roots.
pub fn get_ground(pool: &Pool) -> Result<State> {
    let mut conn = pool.get()?;

    let tx = conn.transaction()?;
    let state = get_roots(&tx)?;

    EventRepository::insert(
        &tx,
        &Event::new(
            "navigate:ground",
            json!({
                "path": "/",
                "state": &state,
            }),
        ),
    )?;

    tx.commit()?;

    Ok(state)
}

/// Returns the folder tree for the given root path.
pub fn get_root(pool: &Pool, path: &str) -> Result<State> {
    let mut conn = pool.get()?;

    if path.starts_with('/') {
        let tx = conn.transaction()?;
        let state = get_root_tree(&tx, path)?;

        tx.commit()?;

        println!("{}", serde_json::to_string_pretty(&state)?);

        EventRepository::insert(
            &conn,
            &Event::new(
                "navigate:root",
                json!({
                    "path": path,
                    "state": &state,
                }),
            ),
        )?;

        Ok(state)
    } else {
        Err(ah!("malformed path"))
    }
}

fn get_root_tree<C>(conn: &C, path: &str) -> Result<State>
where
    C: Deref<Target = Connection>,
{
    let state = State::Tree {
        path: path.to_string(),
        value: get_tree(conn, path)?,
    };

    Ok(state)
}

fn get_roots<C>(conn: &C) -> Result<State>
where
    C: Deref<Target = Connection>,
{
    let state_repo = StateRepository(conn);
    let roots = state_repo.get_roots()?;

    let state = State::Ground { roots };

    Ok(state)
}
// fn vec_compare(v: &[Folder], u: &[Folder]) -> bool {
//     v.len() == u.len() && v.iter().zip(u).all(|(a, b)| a.id == b.id)
// }

fn get_tree<C>(conn: &C, path: &str) -> Result<Tree>
where
    C: Deref<Target = Connection>,
{
    let state_repo = StateRepository(conn);
    let entry = EntryRepository::find_by_path(conn, path)?.unwrap();
    let folder_counts = state_repo.get_folders_with_count(&entry.id)?;

    let tree = if folder_counts.len() > 0 {
        let mut children = vec![];
        for (child_path, count) in folder_counts {
            let child = if count == 0 {
                Tree::Leaf { path: child_path }
            } else {
                get_tree(conn, &child_path)?
            };
            children.push(Box::new(child));
        }

        Tree::Node {
            path: path.to_string(),
            children,
        }
    } else {
        Tree::Leaf {
            path: path.to_string(),
        }
    };

    Ok(tree)
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
            roots: vec![],
            folders: roots,
            assets,
        }
    } else {
        State::Catalogue {
            location,
            roots: vec![],
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
