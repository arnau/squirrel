#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use nut::entities::storage::{params, Pool, Storage};
use nut::services;
// use std::fs;
// use std::io::prelude::*;
// use std::str::FromStr;
// use std::{collections::HashMap, sync::Mutex};
use tauri::Manager;
// use tauri::State;
use tauri::{CustomMenuItem, Menu, MenuEntry, MenuItem, Submenu};

// TODO: Decide whether I need a complementary ephemeral storage.
// struct Storagex {
//     store: Mutex<HashMap<u64, String>>,
// }

#[tauri::command]
async fn locate(route: String, pool: tauri::State<'_, Pool>) -> Result<nut::State, String> {
    match services::navigator::get_path(&pool, &route) {
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
    let pool = services::starter::start("../squirrel.db")?;

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

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![locate, connect,])
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
