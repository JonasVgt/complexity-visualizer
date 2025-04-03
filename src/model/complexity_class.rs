use std::hash::{DefaultHasher, Hash, Hasher};

use crate::database::complexity_class::Tag;

#[allow(dead_code)]
pub struct ComplexityClass {
    pub id: String,
    pub names: Vec<String>,
    pub tags: Vec<Tag>,
    pub description: String,
    pub wikipedia: String,
}

impl ComplexityClass {
    pub fn hash_id(id: &str) -> u64 {
        let mut s = DefaultHasher::new();
        id.hash(&mut s);
        s.finish()
    }

    pub fn calculate_id_hash(&self) -> u64 {
        Self::hash_id(&self.id)
    }
}
