pub mod complexity_class;

use complexity_class::ComplexityClass;
use rusqlite::{Connection, Result};

pub struct MyDatabase {
    conn: Connection,
}

impl MyDatabase {
    pub fn new() -> Self {
        let conn = Connection::open("classes.db").expect("Failed to open database");
        Self { conn }
    }

    pub fn fetch_complexity_classes(&self) -> Result<Vec<ComplexityClass>> {
        let mut stmt = self.conn.prepare("SELECT * FROM complexity_classes")?;
        let classes: Vec<ComplexityClass> = stmt
            .query_map([], |row| {
                Ok(ComplexityClass {
                    id: row.get("id")?,
                    name: row.get("name")?,
                    description: row.get("description")?,
                    wikipedia: row.get("wikipedia_link")?,
                })
            })?
            .collect::<Result<Vec<ComplexityClass>>>()?;

        Ok(classes)
    }
}
