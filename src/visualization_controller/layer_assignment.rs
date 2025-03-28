use std::collections::HashMap;

use petgraph::{graph::NodeIndex, Graph};

use super::layered_graph::LayeredGraph;

pub fn assign_layers<N,E>(graph: Graph<N,E>) -> LayeredGraph<N,E> {
    let mut layer_map: HashMap<NodeIndex, i32> = HashMap::new();
    let mut not_done = Vec::new();

    // Find the roots of the condensed graphs
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
        layer_map.insert(root, 0);
        not_done.push(root);
    }

    while let Some(node) = not_done.pop() {
        let layer = layer_map.get(&node).unwrap().clone();
        for neighbor in graph.neighbors_directed(node, petgraph::Direction::Outgoing) {
            let mut new_layer=layer + 1;
            if let Some(old_layer) = layer_map.get(&neighbor) {
                if old_layer < &new_layer {
                    not_done.push(neighbor);
                }
                new_layer = i32::max(old_layer.clone(), new_layer);
            } else {
                not_done.push(neighbor);
            }
            
            layer_map.insert(neighbor, new_layer);
        }
    }

    let layers: Vec<Vec<NodeIndex>> =
        layer_map
            .into_iter()
            .fold(vec![], |mut accu, (node, level)| {
                if accu.len() < level as usize + 1 {
                    accu.resize(level as usize + 1, vec![]);
                }
                accu[level as usize].push(node);
                accu
            });
    return LayeredGraph::new(graph, layers);
}
