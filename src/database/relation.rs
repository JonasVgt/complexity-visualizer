use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Relation {
    pub id_subset: u32,
    pub id_superset: u32,
    pub relation_type: String,
}