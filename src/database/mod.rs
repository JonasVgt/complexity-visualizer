pub mod complexity_class;
pub mod data;
pub mod relation;

use data::Data;
use flowync::CompactFlower;
use rmp_serde::from_slice;

pub struct MyDatabase {
    flower: CompactFlower<u8, Data, String>,
    pub data: Data,
}

impl MyDatabase {
    pub fn new() -> Self {
        let r = Self {
            flower: CompactFlower::new(1),
            data: Data::new(),
        };
        r.spawn_fetch();
        return r;
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn fetch_complexity_classes() -> Result<Data, std::io::Error> {
        use std::{fs::File, io::Read};

        let mut file = File::open("./assets/classes.msgpack")?;

        // Read the file contents into a buffer
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // Deserialize MessagePack data into Rust struct
        let data: Data = from_slice(&buffer).expect("Failed to deserialize msgpack");

        Ok(data)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn spawn_fetch(&self) {
        let handle = self.flower.handle();

        handle.activate();
        match Self::fetch_complexity_classes() {
            Ok(content) => handle.success(content),
            Err(e) => handle.error(e.to_string()),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn fetch_complexity_classes() -> Result<Data, std::io::Error> {
        use std::io::Error;

        use js_sys::wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        use web_sys::Request;
        use web_sys::RequestInit;
        use web_sys::Response;

        let opts = RequestInit::new();
        opts.set_method("GET");
        //opts.mode(RequestMode::Cors);

        let request =
            Request::new_with_str_and_init("/assets/classes.msgpack", &opts).map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::Other, "Failed to cast response")
            })?;

        // Fetch the MessagePack file via HTTP
        let window = web_sys::window().unwrap();
        let resp = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Failed to fetch file"))?;

        // Check if the response is successful
        assert!(resp.is_instance_of::<Response>());
        let resp: Response = resp.dyn_into().unwrap();
        if !resp.ok() {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Failed to load the MessagePack file",
            ));
        }

        // Get the file content as an ArrayBuffer
        let ab = resp
            .array_buffer()
            .map_err(|_| Error::new(std::io::ErrorKind::Other, "Failed to create array buffer"))?;
        let array_buffer = JsFuture::from(ab)
            .await
            .map_err(|_| Error::new(std::io::ErrorKind::Other, "Failed to read buffer"))?;
        let buffer: Vec<u8> = js_sys::Uint8Array::new(&array_buffer).to_vec();

        let data: Data = from_slice(&buffer).expect("Failed to deserialize msgpack");

        Ok(data)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn spawn_fetch(&self) {
        let handle = self.flower.handle();
        wasm_bindgen_futures::spawn_local(async move {
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
                Ok(data) => {
                    res = true;
                    self.data = data
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
