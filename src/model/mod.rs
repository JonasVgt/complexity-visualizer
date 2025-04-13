pub mod complexity_class;
pub mod filter;
pub mod relation;

use std::collections::HashMap;

use crate::database::{
    self, complexity_class::ComplexityClass as DBComplexityClass, relation::Relation as DBRelation,
};
use complexity_class::{ComplexityClass as ModelComplexityClass, ComplexityClassId};
use egui::ahash::{HashSet, HashSetExt};
use petgraph::{
    algo::{all_simple_paths, has_path_connecting},
    data::DataMap,
    graph::NodeIndex,
    prelude::StableDiGraph,
    visit::{depth_first_search, Control, DfsEvent},
};
use relation::{Relation as ModelRelation, RelationComposition, RelationType, Subset};

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
                let r = match &rel.relation_type {
                    RelationType::Subset(Subset { from, to }) => Some((from, to)),
                    RelationType::Equal(Subset { from, to }, _) => Some((from, to)),
                }
                .map(|(from, to)| (self.get_class(*from).unwrap(), self.get_class(*to).unwrap()));
                r.is_some_and(|(c1, c2)| self.filter.apply_relations(c1, c2))
            })
            .collect()
    }

    pub fn relation_compositions(&self) -> Vec<RelationComposition> {
        let mut graph: StableDiGraph<ComplexityClassId, ModelRelation> = StableDiGraph::new();
        let mut map: HashMap<ComplexityClassId, NodeIndex> = HashMap::new();
        for c in &self.classes {
            let n = graph.add_node(c.id);
            map.insert(c.id, n);
        }

        for r in &self.relations {
            let edges = match r.relation_type {
                RelationType::Subset(s) => vec![(s.from, s.to)],
                RelationType::Equal(s, _) => vec![(s.from, s.to), (s.to, s.from)],
            };
            for (from, to) in edges {
                graph.add_edge(*map.get(&from).unwrap(), *map.get(&to).unwrap(), r.clone());
            }
        }

        let mut filtered_graph = graph.clone();

        filtered_graph.retain_nodes(|gr, n| {
            let id = gr.node_weight(n).unwrap();
            self.filter.apply_classes(self.get_class(*id).unwrap())
        });

        let mut res = HashSet::new();

        for orig_edge in graph.edge_indices() {
            let (from, to) = graph.edge_endpoints(orig_edge).unwrap();

            if filtered_graph.contains_edge(from, to) {
                if has_path_connecting(&graph, to, from, None) {
                    res.insert(RelationComposition::Equalily(vec![graph
                        .edge_weight(orig_edge)
                        .unwrap()
                        .clone()]));
                } else {
                    res.insert(RelationComposition::Subset(vec![graph
                        .edge_weight(orig_edge)
                        .unwrap()
                        .clone()]));
                }
                continue;
            }
            if !filtered_graph.contains_node(from) {
                continue;
            }

            depth_first_search(&graph, Some(to), |event| -> Control<()> {
                if let DfsEvent::TreeEdge(_, v) = event {
                    if filtered_graph.contains_node(v) {
                        if !has_path_connecting(&filtered_graph, from, v, None) {
                            let nodes: Vec<NodeIndex> =
                                all_simple_paths::<Vec<_>, _>(&graph, from, v, 0, None)
                                    .next()
                                    .unwrap();
                            let mut relations = vec![];
                            for i in 1..nodes.len() {
                                let e = graph.find_edge(nodes[i - 1], nodes[i]).unwrap();
                                let rel = graph.edge_weight(e).unwrap();
                                relations.push(rel.clone());
                            }

                            if has_path_connecting(&graph, to, from, None) {
                                res.insert(RelationComposition::Equalily(relations));
                            } else {
                                res.insert(RelationComposition::Subset(relations));
                            }
                        }
                        return Control::Prune;
                    }
                }

                Control::Continue
            });
        }
        res.into_iter().collect()
    }

    pub fn get_relation(
        &self,
        from: ComplexityClassId,
        to: ComplexityClassId,
    ) -> Option<&ModelRelation> {
        self.relations.iter().find(|e| match &e.relation_type {
            RelationType::Equal(Subset { from: f, to: t }, _) => {
                (from == *f && to == *t) || (from == *t && to == *f)
            }
            RelationType::Subset(Subset { from: f, to: t }) => from == *f && to == *t,
        })
    }

    pub fn filter_mut(&mut self) -> &mut filter::Filter {
        &mut self.filter
    }

    fn convert_relations(input: Vec<DBRelation>) -> Vec<ModelRelation> {
        input.into_iter().map(ModelRelation::from).collect()
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
