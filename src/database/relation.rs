use std::hash::{DefaultHasher, Hash, Hasher};

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Relation {
    pub from: String,
    pub to: String,
    pub relation_type: String,
}

impl Relation {
    pub fn calculate_from_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.from.hash(&mut s);
        s.finish()
    }

    pub fn calculate_to_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.to.hash(&mut s);
        s.finish()
    }
}