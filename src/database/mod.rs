mod complexity_class;

use complexity_class::ComplextiyClass;
use rusqlite::{Connection, Result};

pub struct MyDatabase {
    conn: Connection,
}

impl MyDatabase {
    pub fn new() -> Self {
        let conn = Connection::open("classes.db").expect("Failed to open database");
        Self { conn: conn }
    }

    pub fn fetch_complexity_classes(&self) -> Result<Vec<ComplextiyClass>> {
        let mut stmt = self.conn.prepare("SELECT * FROM complexity_classes")?;
        let classes: Vec<ComplextiyClass> = stmt
            .query_map([], |_| {
                Ok(ComplextiyClass {
                    id: 1,
                    name: String::from("abc"),
                    description: String::from("desc"),
                    wikipedia_link: String::from("wikipedia.de"),
                })
            })?
            .collect::<Result<Vec<ComplextiyClass>>>()?;

        Ok(classes)
    }
}
