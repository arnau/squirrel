#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use nut::entities::storage::{params, Pool, Storage};
use nut::services::starter;
// use std::fs;
// use std::io::prelude::*;
// use std::str::FromStr;
// use std::{collections::HashMap, sync::Mutex};
use tauri::Manager;
// use tauri::State;
use tauri::{CustomMenuItem, Menu, MenuEntry, MenuItem, Submenu};

mod catalogue_commands;
mod image_protocol;
mod inspector_commands;
mod preferences_commands;

use catalogue_commands::*;
use image_protocol::image_protocol;
use inspector_commands::*;
use preferences_commands::*;

// TODO: Decide whether I need a complementary ephemeral storage.
// struct Storagex {
//     store: Mutex<HashMap<u64, String>>,
// }

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

fn app_menu(app_name: &str) -> MenuEntry {
    MenuEntry::Submenu(Submenu::new(
        app_name,
        Menu::with_items([
            CustomMenuItem::new("preferences", "Preferences")
                .accelerator("Cmd+,")
                .into(),
            MenuItem::Separator.into(),
            MenuItem::Hide.into(),
            MenuItem::HideOthers.into(),
            MenuItem::ShowAll.into(),
            MenuItem::Separator.into(),
            MenuItem::Services.into(),
            MenuItem::Quit.into(),
        ]),
    ))
}

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

fn go_menu() -> MenuEntry {
    MenuEntry::Submenu(Submenu::new(
        "Go",
        Menu::with_items([CustomMenuItem::new("go:catalogue", "Catalogue")
            .accelerator("Cmd+0")
            .into()]),
    ))
}

fn tools_menu() -> MenuEntry {
    MenuEntry::Submenu(Submenu::new(
        "Tools",
        Menu::with_items([CustomMenuItem::new("tools:inspector", "Inspector")
            .accelerator("Cmd+Shift+i")
            .into()]),
    ))
}

fn window_menu() -> MenuEntry {
    MenuEntry::Submenu(Submenu::new(
        "Window",
        Menu::with_items([
            MenuItem::Minimize.into(),
            MenuItem::EnterFullScreen.into(),
            MenuItem::CloseWindow.into(),
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
    console_subscriber::init();
    let ctx = tauri::generate_context!();
    // TODO: resolve the db path with dirs.
    let db_location = "/Users/arnau/Library/Application Support/net.seachess.squirrel/squirrel.db";
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

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            locate,
            locate_ground,
            locate_folders,
            locate_asset_page,
            fetch_folder_details,
            fetch_thumbnail,

            connect,

            inspect_logs,
            prune_logs,
            open_inspector,

            fetch_preferences,
            store_preference,
            store_connector,
            store_source,
            store_source_connector,
        ])
        .menu(Menu::with_items([
            app_menu(&ctx.package_info().name.to_string()),
            edit_menu(),
            go_menu(),
            window_menu(),
            tools_menu(),
            MenuEntry::Submenu(Submenu::new(
                "Help",
                Menu::with_items([CustomMenuItem::new("learn_more", "Learn More").into()]),
            )),
        ]))
        .on_menu_event(|event| {
            let event_name = event.menu_item_id();
            match event_name {
                "learn_more" => {
                    let link = "https://www.seachess.net/".to_string();
                    tauri::api::shell::open(&event.window().shell_scope(), link, None).unwrap();
                }
                "preferences" => event
                    .window()
                    .emit_to("main", "navigate", "preferences/general")
                    .unwrap(),
                "go:catalogue" => event
                    .window()
                    .emit_to("main", "navigate", "catalogue")
                    .unwrap(),
                "tools:inspector" => {
                    open_inspector2(event.window().app_handle());
                }

                me => {
                    dbg!(me);
                } // _ => {}
            }
        })
        .register_uri_scheme_protocol("image", image_protocol)
        .build(ctx)?;

    app.run(|handle, e| match e {
        // TODO: Cleanup before closing.
        tauri::RunEvent::WindowEvent { label, event, .. } => match event {
            // tauri::WindowEvent::CloseRequested { api, .. } => {
            //     // let inspector close without confirmation
            //     if label == "main" {
            //         let w = handle.get_window("main").unwrap();
            //         tauri::api::dialog::ask(
            //             Some(&w),
            //             "Oh, what do we do?",
            //             "mess a ge",
            //             move |_res| {
            //                 println!("function kaboom");
            //             },
            //         );
            //         println!("mec");
            //     }
            // }
            _we => {
                // dbg!(we);
            }
        },
        tauri::RunEvent::Exit => {
            dbg!("exit");
        }
        _ => {}
    });

    Ok(())
}
