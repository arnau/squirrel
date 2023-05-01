use crate::entities::Result;
use crate::entities::{event::EventLog, storage::Pool};
use crate::repositories::EventRepository;

pub fn get_logs(pool: &Pool, query: &str) -> Result<EventLog> {
    let conn = pool.get()?;

    // TODO: Validate query
    let log = if query.len() > 0 {
        EventRepository::filter(&conn, query)?
    } else {
        EventRepository::get_all(&conn)?
    };

    Ok(log)
}

pub fn prune_logs(pool: &Pool, query: &str) -> Result<EventLog> {
    let conn = pool.get()?;

    // TODO: Validate query
    let log = if query.len() > 0 {
        EventRepository::prune(&conn, query)?
    } else {
        EventRepository::prune_all(&conn)?
    };

    Ok(log)
}
