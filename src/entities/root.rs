use rusqlite::{Result as SQLResult, Row};

#[derive(Debug, Clone)]
pub struct Root {
    pub id: String,
    pub name: String,
    pub path: String,
    pub source_id: String,
}

impl Root {
    pub fn from_row(row: &Row<'_>) -> SQLResult<Root> {
        let id: String = row.get(0)?;
        let name: String = row.get(1)?;
        let path: String = row.get(2)?;
        let source_id: String = row.get(3)?;

        let root = Root {
            id,
            name,
            path,
            source_id,
        };

        Ok(root)
    }
}
