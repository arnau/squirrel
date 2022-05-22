use crate::entities::storage::{params, Connection};
use crate::entities::{Event, Result, Storage};
use crate::repositories::Repository;
use std::ops::Deref;
use std::str::FromStr;

pub struct EventRepository;

impl Repository for EventRepository {}

impl EventRepository {
    pub fn insert<C>(conn: &C, event: &Event) -> Result<()>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
            INSERT INTO event
                (data)
            VALUES
                (?)
            "#;

        let mut stmt = conn.prepare(&query)?;
        let value = event.to_string();
        stmt.execute(params![&value])?;

        Ok(())
    }

    /// Takes the n most recent events.
    pub fn head<C>(conn: &C, amount: usize) -> Result<Vec<Event>>
    where
        C: Deref<Target = Connection>,
    {
        let query = r#"
            SELECT
                data
            FROM
                event
            ORDER BY stamp ASC
            LIMIT ?
            "#;

        let events = Storage::get(conn, &query, params![&amount], |row| {
            let blob: String = row.get(0)?;
            let event: Event = Event::from_str(&blob).expect("event to be parseable");

            Ok(event)
        })?;

        Ok(events)
    }
}
