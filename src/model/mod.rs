pub mod complexity_class;
pub mod filter;
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
use relation::{Relation as ModelRelation, Subset};
use std::collections::HashMap;

pub struct Model {
    relations: Vec<ModelRelation>,
    classes: Vec<ModelComplexityClass>,
    positions: HashMap<u64, Pos2>,
    filter: filter::Filter,
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
            filter: filter::Filter::new(),
        }
    }

    pub fn classes(&self) -> Vec<&ModelComplexityClass> {
        self.classes
            .iter()
            .filter(|c| self.filter.apply_classes(c))
            .collect()
    }

    pub fn get_class(&self, id: u64) -> Option<&ModelComplexityClass> {
        self.classes.iter().find(|e| e.calculate_id_hash() == id)
    }

    pub fn relations(&self) -> Vec<&ModelRelation> {
        self.relations
            .iter()
            .filter(|rel| {
                let r = match rel {
                    ModelRelation::Subset(Subset { from, to }) => Some((from, to)),
                    ModelRelation::Equal(Subset { from, to }, _) => Some((from, to)),
                    ModelRelation::Unknown => None,
                }
                .map(|(from, to)| {
                    (
                        self.get_class(ModelComplexityClass::hash_id(from)).unwrap(),
                        self.get_class(ModelComplexityClass::hash_id(to)).unwrap(),
                    )
                });
                r.is_some_and(|(c1, c2)| self.filter.apply_relations(c1, c2))
            })
            .collect()
    }

    pub fn filter_mut(&mut self) -> &mut filter::Filter {
        &mut self.filter
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

    pub fn update(&mut self) {
        if self.filter.should_redraw() {
            let data = MyDatabase::get_data();
            self.positions = VisualizationController::new(&data).arrange();
            self.filter.redrawn();
        }
    }
}
