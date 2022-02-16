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

fn xxmain() -> Result<(), anyhow::Error> {
    let walker = WalkDir::new("playground").into_iter();

    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "lrcat" {
                    continue;
                }
                if ext == "tif" {
                    continue;
                }
                if ext == "NEF" {
                    continue;
                }
                if ext == "psd" {
                    continue;
                }
            }

            println!("{}", entry.path().display());
            // let access_stamp: DateTime<Utc> = metadata.accessed()?.into();
            // let image = image::open(&entry.path())?;
            // dbg!(access_stamp.to_rfc3339());
            // dbg!(image.dimensions());

            let file = std::fs::File::open(entry.path())?;
            let mut bufreader = std::io::BufReader::new(&file);
            let exifreader = exif::Reader::new();
            let exif = exifreader.read_from_container(&mut bufreader)?;
            for f in exif.fields() {
                println!(
                    "{} {} {}",
                    f.tag,
                    f.ifd_num,
                    f.display_value().with_unit(&exif)
                );
            }

            break;
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let lrprev = "playground/catalogue/2021_JC_Candanedo-v11 Previews.lrdata/0/0977/09776BD3-3D3C-4359-BCD1-3D3C239D37B6-cb4480dde1544872ae29419ec404958c.lrprev";
    let file = fs::File::open(lrprev)?;
    let reader = io::BufReader::new(&file);

    pyramid::extract(reader)?;

    Ok(())
}
