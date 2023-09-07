use crate::Fort;
use crate::entities::Connector;
use crate::entities::connector::{NewConnector, ConnectorId};
use crate::{entities::storage::Pool, Result};
use chrono::Utc;
use serde_json::json;
use crate::repositories::PreferencesRepository;
use anyhow::{anyhow as ah, bail};
use rusqlite::Error as RusqliteError;


type SourceId = String;
type Source = serde_json::Value;


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


/// Stores a new connector putting the given secret key into the SO keychain.
pub fn set_connector(pool: &Pool, new_connector: NewConnector) -> Result<Connector> {
    let fort = Fort::new("squirrel".to_string());
    let mut conn = pool.get()?;
    let tx = conn.transaction()?;
    let preferences_repository = PreferencesRepository(&tx);

    let application_id = format!("{}:{}", &new_connector.id, &new_connector.kind);

    let connector = Connector {
        id: new_connector.id,
        key_name: new_connector.key_name,
        bucket_name: new_connector.bucket_name,
        secret_key: true,
        kind: new_connector.kind,
        creation_stamp: Utc::now().to_rfc3339(),
    };

    preferences_repository.insert_connector(&connector)?;

    // Attempt to set the application key after attempting to store the
    // connector such that if either fails, everything is rolled back;
    fort.set_application_key(&application_id, &new_connector.secret_key)?;

    tx.commit()?;

    Ok(connector)
}

pub fn remove_connector(pool: &Pool, connector_id: ConnectorId) -> Result<()> {
    let fort = Fort::new("squirrel".to_string());
    let mut conn = pool.get()?;
    let tx = conn.transaction()?;
    let preferences_repository = PreferencesRepository(&tx);

    let connector = preferences_repository.get_connector(&connector_id)?;
    let application_id = format!("{}:{}", &connector_id, &connector.kind);

    preferences_repository.delete_connector(&connector_id)?;

    fort.delete_application_key(&application_id)?;

    tx.commit()?;

    Ok(())
}

pub fn set_source(pool: &Pool, source: Source) -> Result<()> {
    Ok(())
}

pub fn set_source_connector(
    pool: &Pool,
    source_id: &SourceId,
    connector_id: &ConnectorId,
) -> Result<()> {
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
    let conn = pool.get()?;
    let preferences_repository = PreferencesRepository(&conn);
    let list = preferences_repository.get_connectors()?;

    let res = json!({
       "id": "connectors",
       "connectors": list,
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
