use nut::entities::event::EventLog;
use nut::entities::storage::Pool;
use nut::services::inspector;
use tauri::{Menu, MenuEntry, MenuEvent, MenuItem, Submenu, WindowBuilder, WindowUrl};

#[tauri::command]
pub async fn open_inspector(handle: tauri::AppHandle) {
    dbg!("inspector!");
    // std::thread::spawn(move || {

    let window = WindowBuilder::new(
        &handle,
        "inspector",
        WindowUrl::App("inspector.html".into()),
    )
    .inner_size(800.0, 800.0)
    .focused(true)
    .build()
    .unwrap();
    #[cfg(debug_assertions)] // only include this code on debug builds
    {
        window.open_devtools();
    }
    // });
}

// #[tauri::command]
pub fn open_inspector2(handle: tauri::AppHandle) {
    // let handle = app.handle();
    std::thread::spawn(move || {
        let window = WindowBuilder::new(
            &handle,
            "inspector",
            WindowUrl::App("inspector.html".into()),
        )
        .inner_size(800.0, 800.0)
        .focused(true)
        .build()
        .unwrap();

        #[cfg(debug_assertions)] // only include this code on debug builds
        {
            window.open_devtools();
        }
    });
}

#[tauri::command]
pub async fn inspect_logs(query: String, pool: tauri::State<'_, Pool>) -> Result<EventLog, String> {
    let res = inspector::get_logs(&pool, &query);

    match res {
        Ok(state) => Ok(state),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn prune_logs(query: String, pool: tauri::State<'_, Pool>) -> Result<EventLog, String> {
    let res = inspector::prune_logs(&pool, &query);

    match res {
        Ok(state) => Ok(state),
        Err(err) => Err(err.to_string()),
    }
}
