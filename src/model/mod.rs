pub mod complexity_class;
pub mod filter;
pub mod relation;

use crate::database::{
    self, complexity_class::ComplexityClass as DBComplexityClass, relation::Relation as DBRelation,
};
use complexity_class::{ComplexityClass as ModelComplexityClass, ComplexityClassId};
use egui::ahash::{HashSet, HashSetExt};
use relation::{Relation as ModelRelation, RelationComposition, Subset};

pub struct Model {
    relations: Vec<ModelRelation>,
    classes: Vec<ModelComplexityClass>,
    filter: filter::Filter,
}

impl Model {
    pub fn new() -> Self {
        let data = database::get_data();
        let relations = Self::convert_relations(data.relations);
        let classes = Self::convert_nodes(data.classes);
        Model {
            relations,
            classes,
            filter: filter::Filter::new(),
        }
    }

    pub fn classes(&self) -> Vec<&ModelComplexityClass> {
        self.classes
            .iter()
            .filter(|c| self.filter.apply_classes(c))
            .collect()
    }

    pub fn get_class(&self, id: ComplexityClassId) -> Option<&ModelComplexityClass> {
        self.classes.iter().find(|e| e.id == id)
    }

    pub fn relations(&self) -> Vec<&ModelRelation> {
        self.relations
            .iter()
            .filter(|rel| {
                let r = match rel {
                    ModelRelation::Subset(Subset { from, to }) => Some((from, to)),
                    ModelRelation::Equal(Subset { from, to }, _) => Some((from, to)),
                }
                .map(|(from, to)| (self.get_class(*from).unwrap(), self.get_class(*to).unwrap()));
                r.is_some_and(|(c1, c2)| self.filter.apply_relations(c1, c2))
            })
            .collect()
    }

    pub fn relation_compositions(&self) -> Vec<RelationComposition> {
        self.relations()
            .iter()
            .map(|r| match r {
                ModelRelation::Subset(_) => RelationComposition::Subset(vec![**r]),
                ModelRelation::Equal(_, _) => RelationComposition::Equalily(vec![**r]),
            })
            .collect()
    }

    pub fn get_relation(
        &self,
        from: ComplexityClassId,
        to: ComplexityClassId,
    ) -> Option<&ModelRelation> {
        self.relations.iter().find(|e| match e {
            ModelRelation::Equal(Subset { from: f, to: t }, _) => {
                (from == *f && to == *t) || (from == *t && to == *f)
            }
            ModelRelation::Subset(Subset { from: f, to: t }) => from == *f && to == *t,
        })
    }

    pub fn filter_mut(&mut self) -> &mut filter::Filter {
        &mut self.filter
    }

    fn convert_relations(input: Vec<DBRelation>) -> Vec<ModelRelation> {
        let converted: Vec<ModelRelation> = input.into_iter().map(ModelRelation::from).collect();
        let mut res = HashSet::new();

        for relation in converted {
            match relation {
                ModelRelation::Subset(s) if res.remove(&ModelRelation::Subset(s.inversed())) => {
                    res.insert(ModelRelation::Equal(s, s.inversed()));
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
                id: a.id.into(),
                names: a.names,
                tags: a.tags,
                description: a.description,
                wikipedia: a.wikipedia,
            })
            .collect()
    }

    pub fn update(&mut self) -> bool {
        if self.filter.should_redraw() {
            self.filter.redrawn();
            return true;
        }
        false
    }
}
