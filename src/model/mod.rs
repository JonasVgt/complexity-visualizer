use std::collections::HashMap;

use egui::Pos2;
use serde::{Deserialize, Serialize};

use crate::{
    database::{complexity_class::ComplexityClass, data::Data, relation::Relation, MyDatabase},
    visualization_controller::VisualizationController,
};

#[derive(Serialize, Deserialize)]
pub struct Model {
    #[serde(skip)]
    db: Option<MyDatabase>,
    data: Data,
    positions: Option<HashMap<u64, Pos2>>,
}

impl Model {
    pub fn new(db: MyDatabase) -> Self {
        return Model {
            data: Data::new(),
            db: Some(db),
            positions: None,
        };
    }

    pub fn classes(&self) -> &Vec<ComplexityClass> {
        return &self.data.classes;
    }

    pub fn get_class(&self, id: u64) -> Option<&ComplexityClass> {
        self.data.classes.iter().find(|e| e.calculate_id_hash() == id)
    }

    pub fn relations(&self) -> &Vec<Relation> {
        return &self.data.relations;
    }

    pub fn get_position(&self, id: &u64) -> Option<&Pos2> {
        self.positions.as_ref()?.get(id)
    }

    pub fn fetch(&mut self) {
        if let Some(mut db) = self.db.take() {
            if db.finish() {
                self.data = db.data;
                self.positions = Some(VisualizationController::new(&self.data).arrange());
            } else {
                self.db = Some(db);
            }
        }
    }
}
