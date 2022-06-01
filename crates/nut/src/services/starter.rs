use crate::entities::catalogue::CatalogueError;
use crate::entities::storage::{Connection, Pool};
use crate::entities::{Event, Result, Storage};
use crate::functions::add_parent_function;
use crate::repositories::{CatalogueRepository, EventRepository};
use crate::VERSION;
use anyhow::anyhow as ah;
use lazy_static::lazy_static;
use serde_json::json;
use std::include_str;
use std::ops::Deref;

lazy_static! {
    static ref BOOTSTRAP: &'static str = include_str!("../storage/catalogue.sql");
    static ref ANALYTICS_BOOT: &'static str = include_str!("../storage/analytics.sql");
}

pub fn start(address: &str) -> Result<Pool> {
    let pool = match address {
        ":memory:" => Storage::memory()?,
        path => Storage::file(path)?,
    };

    setup(&pool.get()?)?;

    Ok(pool)
}

pub fn setup<C>(conn: &C) -> Result<()>
where
    C: Deref<Target = Connection>,
{
    if CatalogueRepository::check_exists(conn)? {
        let version = CatalogueRepository::version(conn)?;

        if version != VERSION {
            let version_pack = version
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(".");
            return Err(ah!(CatalogueError::IncompatibleVersion(version_pack)));
        }

        add_parent_function(&conn)?;
    } else {
        conn.execute_batch(&BOOTSTRAP)?;
        conn.execute_batch(&ANALYTICS_BOOT)?;
        add_parent_function(&conn)?;

        CatalogueRepository::insert_version(conn, VERSION)?;
        CatalogueRepository::insert_pair(conn, "codename", "spip")?;

        EventRepository::insert(
            conn,
            &Event::new("starter:create_catalogue", json!({ "version": VERSION })),
        )?;
    }

    conn.pragma_update(None, "journal_mode", "wal")?;
    conn.pragma_update(None, "foreign_keys", "on")?;

    Ok(())
}
