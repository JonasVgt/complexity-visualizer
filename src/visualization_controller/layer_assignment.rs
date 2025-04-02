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
            if old_layer.is_none() || old_layer.unwrap() < layer + 1 {
                not_done.push(neighbor);
                layer_map.insert(neighbor, layer + 1);
            };
        }
    }

    return LayeredGraph::with_layer_map(graph, layer_map);
}

#[cfg(test)]
mod tests {

    use petgraph::algo::condensation;

    use crate::{
        database::{self, MyDatabase},
        visualization_controller::VisualizationController,
    };

    use super::*;

    fn get_arranged_graph() -> LayeredGraph<Vec<u64>, database::relation::RelationType> {
        let data = MyDatabase::get_data();
        let vc = VisualizationController::new(&data);
        let graph = vc.graph;
        let condensated_graph = condensation(graph, true);
        assign_layers(condensated_graph)
    }

    #[test]
    fn all_nodes_present() {
        let data = MyDatabase::get_data();
        let lg = get_arranged_graph();

        for class in data.classes {
            let hash = class.calculate_id_hash();
            assert!(
                lg.graph().node_weights().any(|nw| { nw.contains(&hash) }),
                "Class: {} with hash {} is not found in graph",
                class.id,
                hash
            );
        }
    }

    #[test]
    fn nodes_in_layers() {
        let lg = get_arranged_graph();
        for node in lg.graph().node_indices() {
            assert!(
                lg.layers().iter().any(|layer| layer.contains(&node)),
                "Node with index {} was not found assigned in layers",
                node.index()
            );
        }
    }

    #[test]
    fn no_duplicate_nodes_in_layers() {
        let lg = get_arranged_graph();
        for node in lg.graph().node_indices() {
            let num_occ = lg
                .layers()
                .into_iter()
                .flatten()
                .filter(|n| **n == node)
                .count();
            assert!(
                num_occ == 1,
                "Node with index {} has {} occurrences in layers, but should only have one",
                node.index(),
                num_occ
            );
        }
    }

    #[test]
    fn nodes_in_layer_map() {
        let lg = get_arranged_graph();
        for node in lg.graph().node_indices() {
            assert!(
                lg.layer_map().contains_key(&node),
                "Node with index {} was not found assigned in layer map",
                node.index()
            );
        }
    }

    #[test]
    fn layers_equals_layermap() {
        let lg = get_arranged_graph();
        assert_eq!(
            lg.layers().into_iter().flatten().count(),
            lg.layer_map().into_iter().count(),
            "Layers and layermap have different amount of elements"
        );

        for node in lg.graph().node_indices() {
            let layer = lg.layer_map().get(&node).unwrap();

            assert!(
                lg.layers()[*layer].contains(&node),
                "Node with index {} is assigned layer {} in layer map, but was not found in that layer",
                node.index(),
                layer
            );
        }
    }

    #[test]
    fn no_reverse_edges() {
        let lg = get_arranged_graph();
        for edge in lg.graph().edge_indices() {
            let (n1, n2) = lg.graph().edge_endpoints(edge).unwrap();

            let layer1 = lg.layer_map().get(&n1).unwrap();
            let layer2 = lg.layer_map().get(&n2).unwrap();

            assert!(
                layer1 != layer2,
                "There is an edge from node: {} to node {}, but both are located in the same layer: {}",
                n1.index(),
                n2.index(),
                layer1
            );

            assert!(
                layer1 < layer2,
                "The edge from node: {} in layer {} to node {} in layer {} is in the wrong direction",
                n1.index(),
                layer1,
                n2.index(),
                layer2
            );
        }
    }

    #[test]
    fn no_empty_layers() {
        let lg = get_arranged_graph();
        for (i, layer) in lg.layers().into_iter().enumerate() {
            assert!(!layer.is_empty(), "Layer {} is empty", i);
        }
    }
}
