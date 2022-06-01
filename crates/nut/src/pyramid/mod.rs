use std::fs;
use std::io;
use walkdir::WalkDir;

pub mod header;
pub mod parser;

pub use header::Header;
pub use parser::extract;
pub use parser::PyramidObject;

use crate::is_hidden;

pub fn process(path: &str) -> anyhow::Result<()> {
    let walker = WalkDir::new(path).into_iter();

    let mut counter = 0;

    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry?;

        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext != "lrprev" {
                    continue;
                }
            }

            println!("{}", entry.path().display());

            let file = fs::File::open(entry.path())?;
            let reader = io::BufReader::new(&file);

            let p = parser::extract(reader)?;
            counter = counter + 1;

            dbg!(p.header);
        }
    }

    println!("total: {}", counter);

    Ok(())
}
