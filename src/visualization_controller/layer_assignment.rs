use std::collections::HashMap;

use petgraph::{graph::NodeIndex, Graph};

use super::layered_graph::LayeredGraph;

pub fn assign_layers<N, E>(graph: Graph<N, E>) -> LayeredGraph<N, E> {
    let mut layer_map: HashMap<NodeIndex, usize> = HashMap::new();
    let mut not_done = Vec::new();

    // Find the roots of the graph
    let roots: Vec<NodeIndex> = graph
        .node_indices()
        .filter(|node| {
            graph
                .neighbors_directed(*node, petgraph::Direction::Incoming)
                .next()
                .is_none()
        })
        .collect();

    for root in roots {
        layer_map.insert(root, 0);
        not_done.push(root);
    }

    while let Some(node) = not_done.pop() {
        let layer = *layer_map.get(&node).unwrap();

        for neighbor in graph.neighbors_directed(node, petgraph::Direction::Outgoing) {
            let old_layer = layer_map.get(&neighbor).copied();
            if old_layer.is_none() || old_layer.unwrap() < layer +1 {
                not_done.push(neighbor);
                layer_map.insert(neighbor, layer+1);
            };
        }
    }

    return LayeredGraph::with_layer_map(graph, layer_map);
}
