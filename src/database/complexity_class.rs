use std::hash::{DefaultHasher, Hash, Hasher};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Tag{
    TIME,
    SPACE,
    DETERMINISTIC,
    NONDETERMINISTIC,
    PROBABILISTIC
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ComplexityClass {
    pub id: String,
    pub names: Vec<String>,
    pub tags: Vec<Tag>,
    pub description: String,
    pub wikipedia: String,
}

impl ComplexityClass {
    pub fn calculate_id_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.id.hash(&mut s);
        s.finish()
    }
}
