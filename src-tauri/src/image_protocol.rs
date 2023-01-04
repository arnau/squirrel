//! The image protocol implements a service able to retrieve thumbnails and images.
//!
//! - Thumbnail pattern: `image://localhost/{image_id}.thumb`
//! - Image pattern: `image://localhost/{image_id}.max`

use nut::entities::storage::Pool;
use nut::services::navigator;
use percent_encoding::percent_decode;
use tauri::http::{Request, Response, ResponseBuilder};
use tauri::AppHandle;
use tauri::{Manager, State};

type ProtocolError = Box<dyn std::error::Error>;

// https://github.com/tauri-apps/wry/blob/dev/examples/custom_protocol.rs
// https://docs.rs/tauri/1.0.0-beta.8/tauri/struct.Builder.html#method.register_uri_scheme_protocol
pub(crate) fn image_protocol(
    app: &AppHandle,
    request: &Request,
) -> Result<Response, ProtocolError> {
    let pool: State<Pool> = app.try_state().expect("couldn't find state pool.");
    // prepare our response
    let mut response = ResponseBuilder::new();
    let route = request
        .uri()
        .strip_prefix("image://localhost/")
        .expect("failed to remove image://localhost/ from the URI.");
    let route = percent_decode(route.as_bytes()).decode_utf8_lossy();

    let blob = match route.rsplit_once('.') {
        Some((id, "max")) => {
            if let Ok(blob) = navigator::get_image(&pool, &id.to_string()) {
                blob
            } else {
                return response.mimetype("text/plain").status(404).body(Vec::new());
            }
        }
        Some((id, "thumb")) => {
            if let Ok(blob) = navigator::get_thumbnail(&pool, &id.to_string()) {
                blob
            } else {
                return response.mimetype("text/plain").status(404).body(Vec::new());
            }
        }

        Some(_) => {
            return response
                .mimetype("text/plain")
                .status(422)
                .body("unknown image size".as_bytes().to_vec())
        }
        None => {
            return response
                .mimetype("text/plain")
                .status(400)
                .body("no image size found".as_bytes().to_vec())
        }
    };

    // Only macOS and Windows are supported, if you set headers in linux they are ignored
    response = response
        .header("Content-Type", "image/jpg")
        .header("Content-Length", blob.data.len());
    // TODO
    // .header("ETag", "hash_from_blob");

    response.mimetype("image/jpeg").status(200).body(blob.data)
}
