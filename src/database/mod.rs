pub mod complexity_class;

use complexity_class::ComplexityClass;
use rmp_serde::from_slice;
use std::{fs::File, io::Read};

pub struct MyDatabase {}

impl MyDatabase {
    pub fn new() -> Self {
        Self {}
    }

    pub fn fetch_complexity_classes(&self) -> Result<Vec<ComplexityClass>, std::io::Error> {
        let mut file = File::open("./assets/classes.msgpack")?;

        // Read the file contents into a buffer
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // Deserialize MessagePack data into Rust struct
        let data: Vec<ComplexityClass> =
            from_slice(&buffer).expect("Failed to deserialize msgpack");

        Ok(data)
    }
}
