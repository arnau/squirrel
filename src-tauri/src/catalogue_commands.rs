use base64::{engine::general_purpose, Engine as _};
use nut::entities::asset::AssetId;
use nut::entities::state::{FolderDetails, Ground, Location, LocationAssetPage, LocationFolders};
use nut::entities::storage::Pool;
use nut::services::navigator;

#[tauri::command]
pub async fn locate(id: String, pool: tauri::State<'_, Pool>) -> Result<Location, String> {
    let res = navigator::locate(&pool, &id);

    match res {
        Ok(state) => Ok(state),
        Err(err) => {
            dbg!(&err);
            Err(err.to_string())
        }
    }
}

#[tauri::command]
pub async fn locate_ground(pool: tauri::State<'_, Pool>) -> Result<Ground, String> {
    let res = navigator::locate_ground(&pool);

    match res {
        Ok(state) => Ok(state),
        Err(err) => {
            dbg!(&err);
            Err(err.to_string())
        }
    }
}

#[tauri::command]
pub async fn locate_folders(
    id: String,
    pool: tauri::State<'_, Pool>,
) -> Result<LocationFolders, String> {
    let res = navigator::locate_folders(&pool, &id);

    match res {
        Ok(state) => Ok(state),
        Err(err) => {
            dbg!(&err);
            Err(err.to_string())
        }
    }
}

#[tauri::command]
pub async fn locate_asset_page(
    id: String,
    cursor: Option<String>,
    pool: tauri::State<'_, Pool>,
) -> Result<LocationAssetPage, String> {
    let res = navigator::locate_asset_page(&pool, &id, cursor);

    match res {
        Ok(state) => Ok(state),
        Err(err) => {
            dbg!(&err);
            Err(err.to_string())
        }
    }
}

#[tauri::command]
pub async fn fetch_folder_details(
    id: String,
    pool: tauri::State<'_, Pool>,
) -> Result<FolderDetails, String> {
    let res = navigator::get_folder_details(&pool, &id);

    match res {
        Ok(state) => Ok(state),
        Err(err) => {
            dbg!(&err);
            Err(err.to_string())
        }
    }
}

#[tauri::command]
pub async fn fetch_thumbnail(id: AssetId, pool: tauri::State<'_, Pool>) -> Result<String, String> {
    let data = if let Ok(blob) = navigator::get_thumbnail(&pool, &id) {
        blob.data
    } else {
        return Err("failed to retrieve thumbnail".into());
    };

    Ok(general_purpose::STANDARD.encode(data))
}

// #[tauri::command]
// async fn thumbnail(id: AssetId, pool: tauri::State<'_, Pool>) -> Result<String, String> {
//     dbg!(&route);
//     let data = if let Ok(blob) = nut::services::navigator::get_async_thumbnail(&pool, &id).await
//     {
//         blob.data
//     } else {
//         dbg!("ai carai!", &route);
//         return Err("failed to retrieve thumbnail".into());
//     };
//
//     Ok(base64::encode(data))
// }

