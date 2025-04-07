use std::fmt;

use serde::{Deserialize, Serialize};

use crate::database::complexity_class::Tag;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct ComplexityClassId([u8; 16]);

impl fmt::Debug for ComplexityClassId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ComplexityClassId")
            .field(
                &std::str::from_utf8(&self.0)
                    .unwrap()
                    .trim_matches(char::from(0)),
            )
            .finish()
    }
}

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
