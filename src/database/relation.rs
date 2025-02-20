use std::hash::{DefaultHasher, Hash, Hasher};

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Relation {
    pub id_subset: String,
    pub id_superset: String,
    pub relation_type: String,
}

impl Relation {
    pub fn calculate_id_subset_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.id_subset.hash(&mut s);
        s.finish()
    }

    pub fn calculate_id_superset_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.id_superset.hash(&mut s);
        s.finish()
    }
}