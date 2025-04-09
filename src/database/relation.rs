use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum RelationType {
    Subset,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Relation {
    pub from: String,
    pub to: String,
    pub relation_type: RelationType,
    pub description: Option<String>,
}
