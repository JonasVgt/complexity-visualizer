use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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

    #[serde(rename = "COMPLEMENT")]
    Complement,
}

impl Tag {
    pub fn tags() -> Vec<Tag> {
        vec![
            Tag::Time,
            Tag::Space,
            Tag::Deterministic,
            Tag::Nondeterministic,
            Tag::Probabilistic,
            Tag::Complement,
        ]
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
            Tag::Complement => "Complement",
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
