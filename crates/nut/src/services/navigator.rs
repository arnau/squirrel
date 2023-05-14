use crate::entities::asset::{AssetId, Blob, BlobSize};
use crate::entities::entry::EntryId;
use crate::entities::state::{
    self, AssetCursor, FolderDetails, Ground, Location, LocationAssetPage, LocationFolders,
};
use crate::entities::storage::{Connection, Pool};
use crate::entities::{Event, Result};
use crate::repositories::{
    AssetRepository, EntryRepository, EventRepository, RootRepository, SourceRepository,
    StateRepository,
};
// use anyhow::anyhow as ah;
use serde_json::json;
use std::ops::Deref;

/// Attempts to locate the resource for the given id.
pub fn locate(pool: &Pool, id: &EntryId) -> Result<Location> {
    let mut conn = pool.get()?;

    let tx = conn.transaction()?;
    let state_repo = StateRepository(&tx);
    let trail = state_repo.get_trail(id)?;
    let path = state_repo.get_location_path(id)?;
    let state = Location {
        id: id.to_string(),
        path,
        trail,
    };

    // TODO: Move out and refactor how to log events.
    // EventRepository::insert(
    //     &tx,
    //     &Event::new(
    //         "locate:entry",
    //         json!({
    //             "id": id.to_string(),
    //         }),
    //     ),
    // )?;

    tx.commit()?;

    Ok(state)
}

/// Attempts to get the list of folders for the given entry id.
pub fn locate_folders(pool: &Pool, parent_id: &EntryId) -> Result<LocationFolders> {
    let mut conn = pool.get()?;
    let tx = conn.transaction()?;

    let state_repo = StateRepository(&tx);
    let folders = state_repo.get_folders(parent_id)?;
    let state = LocationFolders(folders);

    tx.commit()?;

    Ok(state)
}

/// Attempts to get the page of assets for the given entry id and cursor.
///
/// When the cursor is `Option::None`, the first page is retrieved.
pub fn locate_asset_page(
    pool: &Pool,
    parent_id: &EntryId,
    cursor: Option<AssetCursor>,
) -> Result<LocationAssetPage> {
    let mut conn = pool.get()?;
    let tx = conn.transaction()?;

    let state_repo = StateRepository(&tx);
    let (next_cursor, data) = state_repo.get_asset_page(parent_id, cursor)?;
    let state = LocationAssetPage { next_cursor, data };

    tx.commit()?;

    Ok(state)
}

/// Attempts to get the list of roots required for every `Location`.
pub fn locate_ground(pool: &Pool) -> Result<Ground> {
    let mut conn = pool.get()?;
    let tx = conn.transaction()?;

    let state_repo = StateRepository(&tx);
    let roots = state_repo.get_roots()?;
    let state = Ground(roots);

    tx.commit()?;

    Ok(state)
}

pub fn get_folder_details(pool: &Pool, id: &str) -> Result<FolderDetails> {
    let mut conn = pool.get()?;

    let tx = conn.transaction()?;
    let state = get_folder_details_inner(&tx, id)?;

    EventRepository::insert(
        &tx,
        &Event::new(
            "navigate:folder_details",
            json!({
                "id": id,
                "state": &state,
            }),
        ),
    )?;

    tx.commit()?;

    Ok(state)
}

fn get_folder_details_inner<C>(conn: &C, id: &str) -> Result<FolderDetails>
where
    C: Deref<Target = Connection>,
{
    let folder = EntryRepository::find_by_id(conn, id)?.unwrap();
    let folder_count = EntryRepository::find_folder_count(conn, id)?;
    let asset_count = EntryRepository::find_asset_count(conn, id)?;
    let root_repo = RootRepository(conn);
    let source_repo = SourceRepository(conn);
    let root = root_repo.get_by_id(&folder.root_id)?.unwrap();
    let source = source_repo.get_by_id(&folder.source_id)?.unwrap();

    let folder_details = FolderDetails {
        id: folder.id,
        path: folder.path,
        source: state::Source {
            id: folder.source_id,
            name: source.name,
            path: source.path,
            version: source.version,
        },
        root: state::Root {
            id: root.id,
            name: root.name,
            path: root.path,
        },
        folder_count,
        asset_count,
    };

    Ok(folder_details)
}

// TODO: start review

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

// TODO: end review
