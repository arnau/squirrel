use nut::entities::storage::Pool;
use nut::services::configurator;

#[tauri::command]
pub async fn fetch_preferences(
    id: String,
    pool: tauri::State<'_, Pool>,
) -> Result<serde_json::Value, String> {
    let res = configurator::get_section(&pool, &id);

    match res {
        Ok(state) => Ok(state),
        Err(err) => {
            dbg!(&err);
            Err(err.to_string())
        }
    }
}

#[tauri::command]
pub async fn store_preference(
    key: String,
    value: String,
    pool: tauri::State<'_, Pool>,
) -> Result<(), String> {
    let res = configurator::set_preference(&pool, &key, &value);

    match res {
        Ok(state) => Ok(state),
        Err(err) => {
            dbg!(&err);
            Err(err.to_string())
        }
    }
}

#[tauri::command]
pub async fn store_connector(
    connector: serde_json::Value,
    pool: tauri::State<'_, Pool>,
) -> Result<(), String> {
    let res = configurator::set_connector(&pool, connector);

    match res {
        Ok(state) => Ok(state),
        Err(err) => {
            dbg!(&err);
            Err(err.to_string())
        }
    }
}

#[tauri::command]
pub async fn store_source(
    source: serde_json::Value,
    pool: tauri::State<'_, Pool>,
) -> Result<(), String> {
    let res = configurator::set_source(&pool, source);

    match res {
        Ok(state) => Ok(state),
        Err(err) => {
            dbg!(&err);
            Err(err.to_string())
        }
    }
}

#[tauri::command]
pub async fn store_source_connector(
    source_id: String,
    connector_id: String,
    pool: tauri::State<'_, Pool>,
) -> Result<(), String> {
    let res = configurator::set_source_connector(&pool, &source_id, &connector_id);

    match res {
        Ok(state) => Ok(state),
        Err(err) => {
            dbg!(&err);
            Err(err.to_string())
        }
    }
}
