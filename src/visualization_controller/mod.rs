use std::{cmp::min, collections::HashMap};

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

        while let Some(node) = not_done.pop() {
            let id = level_map.get(&node).unwrap() + 1;
            for neighbor in
                condensated_graph.neighbors_directed(node, petgraph::Direction::Incoming)
            {
                if level_map.contains_key(&neighbor) {
                    level_map.insert(neighbor, min(level_map.get(&neighbor).unwrap().clone(), id));
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
            let neighbors: Vec<NodeIndex> = condensated_graph
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
            println!(
                "{:?}",
                unsorted[0]
                    .iter()
                    .map(|node| (node.clone(), heur(node.clone(), done.last().unwrap())))
                    .collect::<Vec<_>>()
            );
            unsorted[0].sort_by_key(|node| heur(node.clone(), done.last().unwrap()));
        }

        let mut map: HashMap<u64, Pos2> = HashMap::new();

        let mut x = 0;
        for level in levels {
            let mut y = 0;

            for node in level
                .iter()
                .flat_map(|node| condensated_graph.node_weight(node.clone()).unwrap())
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
