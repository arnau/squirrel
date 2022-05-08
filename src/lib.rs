use entities::{storage::Connection, Result};
use walkdir::DirEntry;
use std::include_str;
use lazy_static::lazy_static;


pub mod entities;
pub mod pyramid;
pub mod repositories;
pub mod services;
pub mod functions;

lazy_static! {
    static ref BOOTSTRAP: &'static str = include_str!("./storage/catalogue.sql");
}

pub fn bootstrap(conn: &Connection) -> Result<()> {
    conn.execute_batch(&BOOTSTRAP)?;
    functions::add_parent_function(&conn)?;

    Ok(())
}


fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

//TODO
// use anyhow::bail;
// use byteorder::{BigEndian, ReadBytesExt};
// use chrono::{DateTime, Utc};
// use image::{GenericImageView, ImageFormat};
// use std::collections::HashMap;


    // use std::env;
    // use std::time::Instant;

    // let entrypoint = env::args().nth(1).unwrap();
    // // let entrypoint = "/Volumes/homes/greypistachio/GreyPistachio/GP_Photos/Professional_Photos";
    // // let entrypoint = "playground";

    // dbg!(&entrypoint);

    // let now = Instant::now();
    // let walker = WalkDir::new(entrypoint).into_iter();
    // let mut file_counter = 0;
    // let mut dir_counter = 0;

    // for entry in walker.filter_entry(|e| !is_hidden(e)) {
    //     let entry = entry?;

    //     if entry.file_type().is_file() {
    //         file_counter = file_counter + 1;
    //     } else {
    //         dir_counter = dir_counter + 1;
    //     }

    //     println!("{:?}: {}", entry.file_type(), entry.path().display());
    // }


