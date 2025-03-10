use serde::{Deserialize, Serialize};

use super::{complexity_class, relation};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub relations: Vec<relation::Relation>,
    pub classes: Vec<complexity_class::ComplexityClass>,
}