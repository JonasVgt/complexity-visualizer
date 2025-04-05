use core::fmt;
use std::hash::{DefaultHasher, Hash, Hasher};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Tag {
    #[serde(rename = "TIME")]
    Time,

    #[serde(rename = "SPACE")]
    Space,

    #[serde(rename = "DETERMINISTIC")]
    Deterministic,

    #[serde(rename = "NONDETERMINISTIC")]
    Nondeterministic,

    #[serde(rename = "PROBABILISTIC")]
    Probabilistic,
}

impl Tag {
    pub fn tags() -> Vec<Tag> {
        vec![Tag::Time, Tag::Space, Tag:: Deterministic, Tag:: Nondeterministic, Tag::Probabilistic]
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Tag::Deterministic => "Deterministic",
            Tag::Nondeterministic => "Non-deterministic",
            Tag::Probabilistic => "Probabilistic",
            Tag::Space => "Space",
            Tag::Time => "Time",
        })
    }
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
