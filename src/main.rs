use anyhow::bail;
use byteorder::{BigEndian, ReadBytesExt};
use chrono::{DateTime, Utc};
use image::{GenericImageView, ImageFormat};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

mod pyramid;

/// Represents a Run
/// TODO: Find a good name
///
/// It should capture paradata on a Squirrel particular run. For example, the timestamp when
/// Squirrel ran gathering all information.
struct Run {
    id: String,
    timestamp: String,
}

/// Represents the link between an image and the run that gathered it.
struct ImageLog {
    run_id: String,
    image_path: String,
}

/// Represents an image.
#[derive(Debug, Clone)]
struct Image {
    path: PathBuf,
    format: String,             // enum Format
    modification_stamp: String, // SystemTime
    access_stamp: String,
    creation_stamp: String,
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

// fn xxmain() -> Result<(), anyhow::Error> {
//     let walker = WalkDir::new("playground").into_iter();

//     for entry in walker.filter_entry(|e| !is_hidden(e)) {
//         let entry = entry?;
//         let metadata = entry.metadata()?;

//         if entry.file_type().is_file() {
//             if let Some(ext) = entry.path().extension() {
//                 if ext == "lrcat" {
//                     continue;
//                 }
//                 if ext == "tif" {
//                     continue;
//                 }
//                 if ext == "NEF" {
//                     continue;
//                 }
//                 if ext == "psd" {
//                     continue;
//                 }
//             }

//             println!("{}", entry.path().display());
//             // let access_stamp: DateTime<Utc> = metadata.accessed()?.into();
//             // let image = image::open(&entry.path())?;
//             // dbg!(access_stamp.to_rfc3339());
//             // dbg!(image.dimensions());

//             let file = std::fs::File::open(entry.path())?;
//             let mut bufreader = std::io::BufReader::new(&file);
//             let exifreader = exif::Reader::new();
//             let exif = exifreader.read_from_container(&mut bufreader)?;
//             for f in exif.fields() {
//                 println!(
//                     "{} {} {}",
//                     f.tag,
//                     f.ifd_num,
//                     f.display_value().with_unit(&exif)
//                 );
//             }

//             break;
//         }
//     }

//     Ok(())
// }

fn main() -> anyhow::Result<()> {
    use std::env;
    use std::time::Instant;

    let entrypoint = env::args().nth(1).unwrap();
    // let entrypoint = "/Volumes/homes/greypistachio/GreyPistachio/GP_Photos/Professional_Photos";
    // let entrypoint = "playground";

    dbg!(&entrypoint);

    let now = Instant::now();
    let walker = WalkDir::new(entrypoint).into_iter();
    let mut file_counter = 0;
    let mut dir_counter = 0;

    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry?;

        if entry.file_type().is_file() {
            file_counter = file_counter + 1;
        } else {
            dir_counter = dir_counter + 1;
        }

        println!("{:?}: {}", entry.file_type(), entry.path().display());
    }

    println!("total files: {}", file_counter);
    println!("total directories: {}", dir_counter);

    let elapsed_time = now.elapsed();
    println!(
        "Running slow_function() took {} milliseconds.",
        elapsed_time.as_millis()
    );

    Ok(())
}
