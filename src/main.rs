// use anyhow::bail;
// use byteorder::{BigEndian, ReadBytesExt};
// use chrono::{DateTime, Utc};
// use image::{GenericImageView, ImageFormat};
// use std::collections::HashMap;

use squirrel::entities::storage::{Connection, Storage, params};
use squirrel::entities::source::Source;
use squirrel::repositories::source::SourceRepository;

fn main() -> anyhow::Result<()> {
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

    // println!("total files: {}", file_counter);
    // println!("total directories: {}", dir_counter);

    // let elapsed_time = now.elapsed();
    // println!(
    //     "Running slow_function() took {} milliseconds.",
    //     elapsed_time.as_millis()
    // );

    let source1 = Source::from_str(
        "/Users/arnau/kitchen/squirrel/playground/catalogue/2021_JC_Candanedo-v11.lrcat",
    )?;
    let source = Source::from_str(
        "/Users/arnau/kitchen/squirrel/playground/catalogue/2019_JC_Candanedo-v11.lrcat",
    )?;

    dbg!(&source);

    let pool = Storage::file(&source.path)?;

    explore(
        pool.get()?,
        "SELECT id_local, absolutePath FROM AgLibraryRootFolder",
        params![],
        |row| {
            let id: u32 = row.get(0)?;
            let path: String = row.get(1)?;

            Ok((id, path))
        },
    )?;

    let version = SourceRepository::version(pool.get()?)?;
    println!("{}", version);

    Ok(())
}

fn explore<T, P, F>(conn: Connection, query: &str, params: P, f: F) -> Result<()>
where
    P: rusqlite::Params,
    F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
    T: std::fmt::Debug,
{
    let mut stmt = conn.prepare(query)?;
    let root_folders = stmt.query_map(params, f)?;

    for row in root_folders {
        dbg!(row?);
    }

    Ok(())
}
