pub mod complexity_class;
pub mod data;
pub mod relation;

use data::Data;
use rmp_serde::from_slice;

pub struct MyDatabase;

impl MyDatabase {
    pub fn get_data() -> Data {
        from_slice(include_bytes!("../../assets/classes.msgpack")).unwrap()
    }
}
