use serde_json::json;

use crate::{entities::storage::Pool, Result};

pub fn get_section(pool: &Pool, id: &str) -> Result<serde_json::Value> {
    let res = match id {
        "general" => general_section(pool),
        "connectors" => connectors_section(pool),
        "sources" => sources_section(pool),
        _ => unreachable!(),
    };

    res
}

// E.g. set_preference(pool, "download_path", "/foo/bar")
//
// TODO: This function has no way to validate the value is acceptable.
pub fn set_preference(pool: &Pool, key: &str, value: &str) -> Result<()> {
    Ok(())
}

type Connector = serde_json::Value;
pub fn set_connector(pool: &Pool, connector: Connector) -> Result<()> {
    Ok(())
}
type Source = serde_json::Value;
pub fn set_source(pool: &Pool, source: Source) -> Result<()> {
    Ok(())
}

type SourceId = String;
type ConnectorId = String;
pub fn set_source_connector(pool: &Pool, source_id: &SourceId, connector_id: &ConnectorId) -> Result<()> {
    Ok(())
}



fn general_section(pool: &Pool) -> Result<serde_json::Value> {
    // TODO: Fetch info at bootstrap
    // setDownloadDirPath(await downloadDir())

    let res = json!({
        "id": "general",
        "download_path": "/dummy/folder/",
    });

    Ok(res)
}

fn connectors_section(pool: &Pool) -> Result<serde_json::Value> {
    let res = json!({
       "id": "connectors",
       "connectors": [
           {
               "id": "6ad327d8a3d112847a520f17",
               "key_name": "Squirrel-Keyname",
               "bucket_name": "Squirrel-Bucketname",
               "secret_key": true,
               "kind": "backblaze",
               "creation_stamp": "2023-06-12T12:13:14Z",
           }
       ]
    });

    Ok(res)
}

fn sources_section(pool: &Pool) -> Result<serde_json::Value> {
    let res = json!({
       "id": "sources",
       "sources": [
           {
                "id": "source id",
                "name": "source name",
                "path": "source path",
                "version": "source LR version",
                "default_connector_id": "6ad327d8a3d112847a520f17",
                "roots": [
                    {
                        "id": "root id",
                        "name": "root name",
                        "path": "root path",
                        "connector_id": "connector id",
                    }
                ],
                "last_import_stamp": "2023-06-12T10:11:12Z", // From source_event
                "stats": {
                    "folders": 35,
                    "assets": 3400,
                    // TODO: should this be in each root?
                },
           }
       ],
       "connectors": [
           {
               "id": "6ad327d8a3d112847a520f17",
               "name": "Squirrel-Test",
               "secret_key": true,
               "kind": "backblaze",
           }
       ]
    });

    Ok(res)
}
