use crate::entities::asset::{Blob, BlobSize};
use crate::entities::Result;
use crate::pyramid::parser::{self, PyramidObject};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs, io};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pyramid {
    pub filename: String,
    pub path: PathBuf,
    // Images are
    // - AB: correct.
    // - AD: ?
    // - BC: rotated 90 CCW
    // - DA: rotated 90 CW
    //
    //         A
    //     +-------+
    //     |       |
    //   D |       | B
    //     |       |
    //     +-------+
    //         C
    pub orientation: String,
}

impl Pyramid {
    pub fn new<S: Into<String>>(path: PathBuf, filename: S, orientation: S) -> Self {
        Pyramid {
            path,
            filename: filename.into(),
            orientation: orientation.into(),
        }
    }

    pub fn absolute_path(&self) -> PathBuf {
        let nibble = self.filename.get(0..1).unwrap();
        let two_bytes = self.filename.get(0..4).unwrap();
        let pyramid_path = format!("{}/{}/{}", nibble, two_bytes, self.filename);

        self.path.join(&pyramid_path)
    }

    /// Returns the pyramid level blob for the given size.
    ///
    /// ## Exceptions
    ///
    /// This function can fail either because the file does not exist, can't be read or it can't
    /// be parsed.
    /// corrupted (i.e. can't be parsed)
    pub fn blob(&self, size: BlobSize) -> Result<Blob> {
        let file = fs::File::open(&self.absolute_path())?;
        let reader = io::BufReader::new(&file);
        let object = parser::extract(reader)?;
        let blob = match size {
            BlobSize::Max => blob_level(&object, object.len()),
            BlobSize::Thumbnail => blob_level(&object, 2),
        };

        Ok(blob)
    }

    pub async fn async_blob(&self, size: BlobSize) -> Result<Blob> {
        use tokio::fs::File;

        let file = File::open(&self.absolute_path()).await?;
        let object = parser::extract_xxx(file).await?;

        let blob = match size {
            BlobSize::Max => blob_level(&object, object.len()),
            BlobSize::Thumbnail => blob_level(&object, 2),
        };

        Ok(blob)
    }
}

fn blob_level(object: &PyramidObject, level_num: usize) -> Blob {
    let level_key = format!("level_{}", level_num);
    let data = object
        .blobs
        .get(&level_key)
        .expect("pyramid level to exist")
        .clone();
    let level = object.level(level_num);

    Blob {
        data,
        height: level.height as usize,
        width: level.width as usize,
    }
}
