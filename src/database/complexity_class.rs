use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ComplexityClass {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub wikipedia: String,
}
