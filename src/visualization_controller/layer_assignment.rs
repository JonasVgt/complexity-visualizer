use std::collections::HashMap;

use petgraph::{graph::NodeIndex, Graph};
use std::cmp::min;

pub fn assign_layers<N,E>(graph: &Graph<N,E>) -> Vec<Vec<NodeIndex>> {
    let mut level_map: HashMap<NodeIndex, i32> = HashMap::new();
    let mut not_done = Vec::new();

    // Find the leafs of the condensed graphs
    let roots: Vec<NodeIndex> = graph
        .node_indices()
        .filter(|node| {
            graph
                .neighbors_directed(node.clone(), petgraph::Direction::Incoming)
                .next()
                .is_none()
        })
        .collect();

    for root in roots {
        level_map.insert(root, 0);
        not_done.push(root);
    }

    while let Some(node) = not_done.pop() {
        let id = level_map.get(&node).unwrap() + 1;
        for neighbor in graph.neighbors_directed(node, petgraph::Direction::Outgoing) {
            if level_map.contains_key(&neighbor) {
                let old_id = level_map.get(&neighbor).unwrap().clone();
                let new_id = min(old_id, id);

                level_map.insert(neighbor, new_id);
            } else {
                level_map.insert(neighbor, id);
                not_done.push(neighbor);
            }
        }
    }

    let levels: Vec<Vec<NodeIndex>> =
        level_map
            .into_iter()
            .fold(vec![], |mut accu, (node, level)| {
                if accu.len() < level as usize + 1 {
                    accu.resize(level as usize + 1, vec![]);
                }
                accu[level as usize].push(node);
                accu
            });
            return levels;
}
