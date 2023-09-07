use crate::entities::connector::ConnectorId;
use crate::entities::storage::{params, Connection, Storage};
use crate::entities::{Result, Connector};
use crate::repositories::Repository;
use std::ops::Deref;

pub struct PreferencesRepository<'c, Conn: Deref<Target = Connection>>(pub &'c Conn);

impl<'c, Conn: Deref<Target = Connection>> Repository for PreferencesRepository<'c, Conn> {}

impl<'c, Conn: Deref<Target = Connection>> PreferencesRepository<'c, Conn> {
    pub fn get_connectors(&self) -> Result<Vec<Connector>> {
        let query = r#"
        SELECT
            id,
            key_name,
            bucket_name,
            secret_key,
            kind,
            creation_stamp
        FROM
            connector
        "#;

        Storage::get(self.0, query, params![], |row| {
            let connector = Connector::from_row(row)?;

            Ok(connector)
        })
    }

    pub fn get_connector(&self, connector_id: &ConnectorId) -> Result<Connector> {
        let query = r#"
        SELECT
            id,
            key_name,
            bucket_name,
            secret_key,
            kind,
            creation_stamp
        FROM
            connector
        WHERE
            id = ?
        "#;

        Storage::get_one(self.0, query, params![connector_id], |row| {
            let connector = Connector::from_row(row)?;

            Ok(connector)
        })
    }

    pub fn insert_connector(&self, connector: &Connector) -> Result<()> {
        let query = r#"
        INSERT INTO connector
            (
                id,
                key_name,
                bucket_name,
                secret_key,
                kind,
                creation_stamp
            )
        VALUES
            (?, ?, ?, ?, ?, ?)
        "#;

       let mut stmt = self.0.prepare(query)?;
        stmt.execute(params![
            &connector.id,
            &connector.key_name,
            &connector.bucket_name,
            &connector.secret_key,
            &connector.kind,
            &connector.creation_stamp,
        ])?;

        Ok(())
    }

    pub fn delete_connector(&self, connector_id: &ConnectorId) -> Result<()> {
        let query = r#"
        DELETE FROM
            connector
        WHERE
            id = ?
        "#;

       let mut stmt = self.0.prepare(query)?;
        stmt.execute(params![connector_id])?;

        Ok(())
    }


}
