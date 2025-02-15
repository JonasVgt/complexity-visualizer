pub mod complexity_class;

use complexity_class::ComplexityClass;
use flowync::CompactFlower;
use rmp_serde::from_slice;
use std::{fs::File, io::Read};
use tokio::runtime;

pub struct MyDatabase {
    rt: runtime::Runtime,
    pub flower: CompactFlower<ComplexityClass, Vec<ComplexityClass>, String>,
    pub classes: Vec<ComplexityClass>,
}

impl MyDatabase {
    pub fn new() -> Self {
        let r = Self {
            rt: runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
            flower: CompactFlower::new(1),
            classes: vec![],
        };
        r.spawn_fetch();
        return r;
    }

    pub async fn fetch_complexity_classes() -> Result<Vec<ComplexityClass>, std::io::Error> {
        let mut file = File::open("./assets/classes.msgpack")?;

        // Read the file contents into a buffer
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // Deserialize MessagePack data into Rust struct
        let data: Vec<ComplexityClass> =
            from_slice(&buffer).expect("Failed to deserialize msgpack");

        Ok(data)
    }

    pub fn spawn_fetch(&self) {
        let handle = self.flower.handle();

        self.rt.spawn(async move {
            handle.activate();

            match Self::fetch_complexity_classes().await {
                Ok(content) => handle.success(content),
                Err(e) => handle.error(e.to_string()),
            }
        });
    }

    pub fn finish(&mut self) -> bool {
        let mut res = false;
        if self.flower.result_is_ready() {
            self.flower.poll(|_| {}).finalize(|result| match result {
                Ok(cc) => {
                    res = true;
                    self.classes = cc
                }
                Err(err) => {
                    res = false;
                    println!("Error: {:?}", err)
                }
            });
        }
        return res;
    }
}
