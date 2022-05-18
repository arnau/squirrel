use crate::entities::storage::{ Pool, Connection };
use crate::entities::{Event, Result};
use crate::functions::add_parent_function;
use crate::repositories::{EventRepository};
use anyhow::anyhow as ah;
use lazy_static::lazy_static;
use serde_json::json;
use std::include_str;
use std::ops::Deref;

lazy_static! {
    static ref BOOTSTRAP: &'static str = include_str!("../storage/catalogue.sql");
}

// Plan:
//
// - Get config
// - Check whether database file exists
//   - Create dir structure
//   - Create db file
// - Connect
// - Setup
pub fn start() -> Result<()> {
    Ok(())
}

pub fn setup<C>(conn: &C) -> Result<()>
where
    C: Deref<Target = Connection>,
{
    conn.execute_batch(&BOOTSTRAP)?;
    add_parent_function(&conn)?;
    // TODO: insert version

    conn.pragma_update(None, "journal_mode", "wal")?;
    conn.pragma_update(None, "foreign_keys", "on")?;


    Ok(())
}
