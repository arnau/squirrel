#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use nut::entities::asset::AssetId;
use nut::entities::storage::{params, Pool, Storage};
use nut::services::{inspector, navigator, starter};
// use std::fs;
// use std::io::prelude::*;
// use std::str::FromStr;
// use std::{collections::HashMap, sync::Mutex};
use tauri::{Manager, WindowBuilder, WindowUrl};
// use tauri::State;
use base64::{engine::general_purpose, Engine as _};
use tauri::{CustomMenuItem, Menu, MenuEntry, MenuItem, Submenu};

mod image_protocol;
use image_protocol::image_protocol;

// TODO: Decide whether I need a complementary ephemeral storage.
// struct Storagex {
//     store: Mutex<HashMap<u64, String>>,
// }

#[tauri::command]
async fn open_inspector(app: tauri::AppHandle) {
    let window = WindowBuilder::new(&app, "inspector", WindowUrl::App("inspector".into()))
        .build()
        .unwrap();
}

#[tauri::command]
async fn inspect_logs(
    query: String,
    pool: tauri::State<'_, Pool>,
) -> Result<nut::entities::event::EventLog, String> {
    let res = inspector::get_logs(&pool, &query);

    match res {
        Ok(state) => Ok(state),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn prune_logs(
    query: String,
    pool: tauri::State<'_, Pool>,
) -> Result<nut::entities::event::EventLog, String> {
    let res = inspector::prune_logs(&pool, &query);

    match res {
        Ok(state) => Ok(state),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn locate(route: String, pool: tauri::State<'_, Pool>) -> Result<nut::State, String> {
    let res = navigator::get_path(&pool, &route);

    match res {
        Ok(state) => Ok(state),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn fetch_ground(pool: tauri::State<'_, Pool>) -> Result<nut::State, String> {
    let res = navigator::get_ground(&pool);

    match res {
        Ok(state) => Ok(state),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn fetch_root(path: String, pool: tauri::State<'_, Pool>) -> Result<nut::State, String> {
    let res = navigator::get_root(&pool, &path);

    match res {
        Ok(state) => Ok(state),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn fetch_route(route: String, pool: tauri::State<'_, Pool>) -> Result<nut::State, String> {
    let res = navigator::get_route(&pool, &route);

    match res {
        Ok(state) => Ok(state),
        Err(err) => Err(err.to_string()),
    }
}

// #[tauri::command]
// async fn state() -> Vec<u8> {
//     let p = std::path::PathBuf::from_str("..").unwrap();
//     dbg!(p.canonicalize());
//     let mut f = fs::File::open("../bald_man.png").unwrap();
//     let mut data = Vec::new();
//     f.read_to_end(&mut data).unwrap();
//     // base64::encode(data)
//     data
// }

#[tauri::command]
async fn thumbnail(id: AssetId, pool: tauri::State<'_, Pool>) -> Result<String, String> {
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

fn edit_menu() -> MenuEntry {
    MenuEntry::Submenu(Submenu::new(
        "Edit",
        Menu::with_items([
            MenuItem::Undo.into(),
            MenuItem::Redo.into(),
            MenuItem::Separator.into(),
            MenuItem::Cut.into(),
            MenuItem::Copy.into(),
            MenuItem::Paste.into(),
            MenuItem::SelectAll.into(),
        ]),
    ))
}

#[tauri::command]
fn connect(pool: tauri::State<Pool>) {
    let conn = pool.get().unwrap();

    let query = r#"
    select
        name,
        type
   from
        pragma_table_list
    "#;

    let x = Storage::get(&conn, &query, params![], |row| {
        let name: String = row.get(0)?;
        let kind: String = row.get(1)?;

        Ok((name, kind))
    })
    .unwrap();

    dbg!(x);
}

// #[tauri::command]
// fn storage_insert(key: u64, value: String, storage: State<Storagex>) {
//     // mutate the storage behind the Mutex
//     storage.store.lock().unwrap().insert(key, value);
//
//     dbg!(storage.store.lock().unwrap().len());
// }

fn main() -> anyhow::Result<()> {
    let ctx = tauri::generate_context!();
    // TODO: resolve the db path with dirs.
    let db_location = "/Users/arnau/Library/ApplicationSupport/net.seachess.squirrel/squirrel.db";
    dbg!(&db_location);
    let pool = starter::start(db_location)?;

    let app = tauri::Builder::default()
        // .manage(Storagex {
        //     store: Default::default(),
        // })
        .manage(pool)
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }

            // let main_window = app.get_window("main").unwrap();
            // tauri::api::dialog::message(Some(&main_window), "Hello", "Jo t'estimo més!!");

            let handle = app.handle();
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
                window.open_devtools();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            locate,
            fetch_ground,
            fetch_root,
            fetch_route,
            connect,
            thumbnail,
            inspect_logs,
            prune_logs,
            open_inspector,
        ])
        .menu(Menu::with_items([
            MenuEntry::Submenu(Submenu::new(
                &ctx.package_info().name,
                Menu::with_items([MenuItem::Services.into(), MenuItem::Quit.into()]),
            )),
            edit_menu(),
            MenuEntry::Submenu(Submenu::new(
                "Help",
                Menu::with_items([CustomMenuItem::new("Learn More", "Learn More").into()]),
            )),
        ]))
        .on_menu_event(|event| {
            let event_name = event.menu_item_id();
            match event_name {
                "Learn More" => {
                    let link = "https://www.seachess.net/".to_string();
                    tauri::api::shell::open(&event.window().shell_scope(), link, None).unwrap();
                }
                _ => {}
            }
        })
        .register_uri_scheme_protocol("image", image_protocol)
        .build(ctx)?;
    // .expect("error while running tauri application");

    app.run(|handle, e| match e {
        tauri::RunEvent::Exit => {
            let w = handle.get_window("main").unwrap();
            tauri::api::dialog::ask(Some(&w), "Oh, what do we do?", "mess a ge", move |res| {
                println!("function kaboom");
            });
            println!("mec");
        }
        _ => {}
    });

    Ok(())
}
