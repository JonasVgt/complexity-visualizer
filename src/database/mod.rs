pub mod complexity_class;

use complexity_class::ComplextiyClass;
use rusqlite::{Connection, Result};

pub struct MyDatabase {
    conn: Connection,
}

impl MyDatabase {
    pub fn new() -> Self {
        let conn = Connection::open("classes.db").expect("Failed to open database");
        Self { conn }
    }

    pub fn fetch_complexity_classes(&self) -> Result<Vec<ComplextiyClass>> {
        let mut stmt = self.conn.prepare("SELECT * FROM complexity_classes")?;
        let classes: Vec<ComplextiyClass> = stmt
            .query_map([], |row| {
                Ok(ComplextiyClass {
                    id: row.get("id")?,
                    name: row.get("name")?,
                    description: row.get("description")?,
                    wikipedia_link: row.get("wikipedia_link")?,
                })
            })?
            .collect::<Result<Vec<ComplextiyClass>>>()?;

        Ok(classes)
    }
}
