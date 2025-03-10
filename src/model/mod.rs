use std::collections::HashMap;

use egui::Pos2;
use serde::{Deserialize, Serialize};

use crate::{
    database::{complexity_class::ComplexityClass, data::Data, relation::Relation, MyDatabase},
    visualization_controller::VisualizationController,
};

#[derive(Serialize, Deserialize)]
pub struct Model {
    data: Data,
    positions: HashMap<u64, Pos2>,
}

impl Model {
    pub fn new() -> Self {
        let data = MyDatabase::get_data();
        let positions = VisualizationController::new(&data).arrange();
        return Model { data, positions };
    }

    pub fn classes(&self) -> &Vec<ComplexityClass> {
        return &self.data.classes;
    }

    pub fn get_class(&self, id: u64) -> Option<&ComplexityClass> {
        self.data
            .classes
            .iter()
            .find(|e| e.calculate_id_hash() == id)
    }

    pub fn relations(&self) -> &Vec<Relation> {
        return &self.data.relations;
    }

    pub fn get_position(&self, id: &u64) -> Option<&Pos2> {
        self.positions.get(id)
    }
}
