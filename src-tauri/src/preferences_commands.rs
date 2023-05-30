use nut::entities::storage::Pool;
use serde_json::json;



#[tauri::command]
pub async fn fetch_pref_section(id: String, pool: tauri::State<'_, Pool>) -> Result<serde_json::Value, String> {
    dbg!(&id);
    let res = json!({
       "id": id,
        "title": "General",
        "download_folder": "/dummy/folder/",
    });


    Ok(res)
}
