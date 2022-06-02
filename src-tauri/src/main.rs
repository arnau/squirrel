#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use std::io::prelude::*;
use std::str::FromStr;
use tauri::Manager;
use tauri::{CustomMenuItem, Menu, MenuEntry, MenuItem, Submenu};

mod ui {
    use std::str::FromStr;

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Thumbnail(String);

    #[derive(Serialize, Deserialize)]
    pub struct Image(String);

    #[derive(Serialize, Deserialize)]
    pub struct File {
        pub path: String,
        // The catalogue where the file was sourced from. Should be a Source with relevant
        // information.
        // pub source: String,
        // or extension
        // pub file_type: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Folder {
        pub path: String,
        // pub source: String,
    }

    // Location should capture the metadata for the current entity.
    //
    // For files:
    //
    // - id (the absolute route)
    // - file_type
    // - classification (star, colour)
    // - bb download. Actually, this is not needed because the action will happen in the
    // backend.
    // - modification time
    //
    // For folders:
    //
    // - id (the absolute route)
    // - modification time?
    #[derive(Serialize, Deserialize)]
    #[serde(tag = "kind")]
    pub enum Location {
        File(File),
        Folder(Folder),
    }

    impl FromStr for Location {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s.find('.').is_some() {
                return Ok(Location::File(File {
                    path: s.to_string(),
                }));
            }

            Ok(Location::Folder(Folder {
                path: s.to_string(),
            }))
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct State {
        pub location: Location,
        // parent: Option<String>,
        // folders: Vec<String>, // a set of routes that can be promoted to a Location
        // thumbnails: Vec<Thumbnail>, // a set of routes + bytes for thumbnails.
        // selected_folder: Option<u32>,

        // these 2 could be collapsed into one. If a thumbnail is selected, an
        // image with the same path/id must be selected.
        // selected_thumbnail: Option<u32>,
        // selected_image: Option<Image>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct World {
        history: Vec<State>,
        current: State,
    }
}

#[tauri::command]
async fn locate(location: String) -> Result<ui::State, String> {
    Ok(ui::State {
        location: ui::Location::from_str(&location)?,
    })
}

#[tauri::command]
async fn state() -> Vec<u8> {
    let p = std::path::PathBuf::from_str("..").unwrap();
    dbg!(p.canonicalize());
    let mut f = fs::File::open("../bald_man.png").unwrap();
    let mut data = Vec::new();
    f.read_to_end(&mut data).unwrap();
    // base64::encode(data)
    data
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

fn main() -> anyhow::Result<()> {
    let ctx = tauri::generate_context!();

    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![locate, state])
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
