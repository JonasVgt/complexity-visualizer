use serde::{Deserialize, Serialize};

use crate::database::complexity_class::Tag;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct ComplexityClassId([u8; 16]);

impl From<String> for ComplexityClassId {
    fn from(value: String) -> Self {
        let mut id = [0u8; 16];
        let len = value.len().min(16);
        id[..len].copy_from_slice(&value.as_bytes()[..len]); // Copy the available bytes
        ComplexityClassId(id)
    }
}
#[allow(dead_code)]
pub struct ComplexityClass {
    pub id: ComplexityClassId,
    pub names: Vec<String>,
    pub tags: Vec<Tag>,
    pub description: String,
    pub wikipedia: String,
}

impl ComplexityClass {}
