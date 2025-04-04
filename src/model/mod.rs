pub mod complexity_class;
pub mod relation;

use crate::{
    database::{
        complexity_class::ComplexityClass as DBComplexityClass, relation::Relation as DBRelation,
        MyDatabase,
    },
    visualization_controller::VisualizationController,
};
use complexity_class::ComplexityClass as ModelComplexityClass;
use egui::{
    ahash::{HashSet, HashSetExt},
    Pos2,
};
use relation::Relation as ModelRelation;
use std::collections::HashMap;

pub struct Model {
    relations: Vec<ModelRelation>,
    classes: Vec<ModelComplexityClass>,
    positions: HashMap<u64, Pos2>,
}

impl Model {
    pub fn new() -> Self {
        let data = MyDatabase::get_data();
        let positions = VisualizationController::new(&data).arrange();
        let relations = Self::convert_relations(data.relations);
        let classes = Self::convert_nodes(data.classes);
        Model {
            relations,
            classes,
            positions,
        }
    }

    pub fn classes(&self) -> &Vec<ModelComplexityClass> {
        &self.classes
    }

    pub fn get_class(&self, id: u64) -> Option<&ModelComplexityClass> {
        self.classes.iter().find(|e| e.calculate_id_hash() == id)
    }

    pub fn relations(&self) -> &Vec<ModelRelation> {
        &self.relations
    }

    pub fn get_position(&self, id: &u64) -> Option<&Pos2> {
        self.positions.get(id)
    }

    fn convert_relations(input: Vec<DBRelation>) -> Vec<ModelRelation> {
        let converted: Vec<ModelRelation> = input.into_iter().map(ModelRelation::from).collect();
        let mut res = HashSet::new();

        for relation in converted {
            match relation {
                ModelRelation::Subset(s)
                    if res.remove(&ModelRelation::Subset(s.clone().inversed())) =>
                {
                    res.insert(ModelRelation::Equal(s.clone(), s.clone().inversed()));
                }
                a => {
                    res.insert(a);
                }
            };
        }
        res.into_iter().collect()
    }

    fn convert_nodes(input: Vec<DBComplexityClass>) -> Vec<ModelComplexityClass> {
        input
            .into_iter()
            .map(|a| ModelComplexityClass {
                id: a.id,
                names: a.names,
                tags: a.tags,
                description: a.description,
                wikipedia: a.wikipedia,
            })
            .collect()
    }
}
