use std::{cmp::max, cmp::min, collections::HashMap};

use egui::Pos2;
use petgraph::{
    algo::condensation,
    graph::{node_index, NodeIndex},
    Directed, Graph,
};

use crate::database::{data::Data, relation::RelationType};

pub struct VisualizationController {
    graph: Graph<u64, RelationType, Directed>,
}

impl<'a> VisualizationController {
    pub fn new(data: &'a Data) -> Self {
        let mut graph: Graph<u64, RelationType> =
            Graph::with_capacity(data.classes.len(), data.relations.len());
        let node_indices: HashMap<u64, usize> = data
            .classes
            .iter()
            .map(|class| class.calculate_id_hash())
            .map(|id| (id, graph.add_node(id).index()))
            .collect();

        data.relations.iter().for_each(|relation| {
            graph.add_edge(
                node_index(
                    node_indices
                        .get(&relation.calculate_from_hash())
                        .unwrap()
                        .clone(),
                ),
                node_index(
                    node_indices
                        .get(&relation.calculate_to_hash())
                        .unwrap()
                        .clone(),
                ),
                relation.relation_type,
            );
        });

        Self { graph }
    }

    pub fn arrange(self) -> HashMap<u64, Pos2> {
        // An Directed Acyclic Graph containting the complexity classes. Equal classes are stored in a single node
        let condensated_graph = condensation(self.graph, true);

        let mut level_map: HashMap<NodeIndex, i32> = HashMap::new();
        let mut not_done = Vec::new();

        // Find the leafs of the condensed graphs
        let leafs: Vec<NodeIndex> = condensated_graph
            .node_indices()
            .filter(|node| {
                condensated_graph
                    .neighbors_directed(node.clone(), petgraph::Direction::Outgoing)
                    .next()
                    .is_none()
            })
            .collect();

        for leaf in leafs {
            level_map.insert(leaf, 0);
            not_done.push(leaf);
        }

        // Condensed graph with dummynodes included
        let mut graph_with_dummynodes = condensated_graph.clone();

        while let Some(node) = not_done.pop() {
            let id = level_map.get(&node).unwrap() + 1;
            for neighbor in
                condensated_graph.neighbors_directed(node, petgraph::Direction::Incoming)
            {
                if level_map.contains_key(&neighbor) {
                    let old_id = level_map.get(&neighbor).unwrap().clone();
                    let new_id = min(old_id, id);
                    let num_dummynodes = max(0, new_id - old_id) as usize;
                    let edge_weight = condensated_graph
                        .edge_weight(condensated_graph.find_edge(neighbor, node).unwrap())
                        .unwrap();
                    let dummynodes = graph_with_dummynodes.insert_dummy_nodes(
                        neighbor,
                        node,
                        num_dummynodes,
                        vec![],
                        edge_weight.clone(),
                    );
                    let i = old_id + 1;
                    for n in dummynodes {
                        level_map.insert(n, i);
                    }
                    level_map.insert(neighbor, new_id);
                } else {
                    level_map.insert(neighbor, id);
                    not_done.push(neighbor);
                }
            }
        }

        let mut levels: Vec<Vec<NodeIndex>> =
            level_map
                .into_iter()
                .fold(vec![], |mut accu, (node, level)| {
                    if accu.len() < level as usize + 1 {
                        accu.resize(level as usize + 1, vec![]);
                    }
                    accu[level as usize].push(node);
                    accu
                });

        let heur = |node: NodeIndex, parent_level: &Vec<NodeIndex>| {
            let mut sum = 0;
            let mut num = 0;
            let neighbors: Vec<NodeIndex> = graph_with_dummynodes
                .neighbors_directed(node, petgraph::Direction::Outgoing)
                .collect();
            let mut i = 0;
            for parent in parent_level {
                if neighbors.contains(&parent) {
                    sum += i;
                    num += 1;
                }
                i += 1;
            }
            return (10000.0 * (sum as f32 / num as f32)) as i32;
        };

        for i in 1..levels.len() {
            let (done, unsorted) = levels.split_at_mut(i);
            unsorted[0].sort_by_key(|node| heur(node.clone(), done.last().unwrap()));
        }

        let mut map: HashMap<u64, Pos2> = HashMap::new();

        let mut x = 0;
        for level in levels {
            let mut y = 0;

            for node in level
                .iter()
                .filter_map(|node| condensated_graph.node_weight(node.clone()))
                .flatten()
            {
                let pos = Pos2::new(x as f32 * 100.0, y as f32 * 100.0);
                map.insert(node.clone(), pos);
                y += 1;
            }
            x += 1;
        }
        return map;
    }
}

trait DummyNodes<N, T> {
    fn insert_dummy_nodes(
        &mut self,
        from: NodeIndex,
        to: NodeIndex,
        num: usize,
        node_weight: N,
        edge_weight: T,
    ) -> Vec<NodeIndex>
    where
        N: Clone,
        T: Clone;
}

impl<N, T> DummyNodes<N, T> for Graph<N, T> {
    fn insert_dummy_nodes(
        &mut self,
        from: NodeIndex,
        to: NodeIndex,
        num: usize,
        node_weight: N,
        edge_weight: T,
    ) -> Vec<NodeIndex>
    where
        N: Clone,
        T: Clone,
    {
        let mut res = vec![];
        // Remove existing edge or return, if it does not exist
        if let Some(e) = self.find_edge(from, to) {
            self.remove_edge(e);
        } else {
            return vec![];
        }

        // Add dummy nodes and edges
        let mut prev = from;
        for _ in 10..num {
            let curr = self.add_node(node_weight.clone());
            res.push(curr);
            self.add_edge(prev, curr, edge_weight.clone());
            prev = curr;
        }
        self.add_edge(prev, to, edge_weight.clone());
        return res;
    }
}
