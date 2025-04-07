use petgraph::graph::NodeIndex;

use super::layered_graph::LayeredGraph;

impl<N, E> LayeredGraph<N, E> {
    fn insert_dummy_nodes(
        &mut self,
        from: NodeIndex,
        to: NodeIndex,
        from_layer: usize,
        to_layer: usize,
        node_weight: N,
        edge_weight: E,
    ) -> Vec<NodeIndex>
    where
        N: Clone,
        E: Clone,
    {
        let mut res = vec![];
        // Remove existing edge or return, if it does not exist
        if let Some(e) = self.graph().find_edge(from, to) {
            self.remove_edge(e);
        } else {
            println!("ERROR, no edge");
            return vec![];
        }

        // Add dummy nodes and edges
        let mut prev = from;
        for layer in (from_layer + 1)..to_layer {
            let curr = self.add_node(node_weight.clone(), layer);
            res.push(curr);
            self.add_edge(prev, curr, edge_weight.clone());
            prev = curr;
        }
        self.add_edge(prev, to, edge_weight.clone());
        res
    }

    pub fn add_dummy_nodes(mut self, dummy_node_weight: N) -> Self
    where
        E: Clone,
        N: Clone,
    {
        let edges = self
            .graph()
            .edge_indices()
            .filter_map(|e| self.graph().edge_endpoints(e))
            .collect::<Vec<_>>();

        for (from, to) in edges {
            let from_layer = *self.layer_map().get(&from).unwrap();
            let to_layer = *self.layer_map().get(&to).unwrap();

            if to_layer <= from_layer + 1 {
                continue;
            }
            let edge = self.graph().find_edge(from, to).unwrap();
            let edge_weight = self.graph().edge_weight(edge).unwrap();

            self.insert_dummy_nodes(
                from,
                to,
                from_layer,
                to_layer,
                dummy_node_weight.clone(),
                edge_weight.clone(),
            );
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use petgraph::algo::{condensation, has_path_connecting};

    use crate::{
        database::{self},
        model::complexity_class::ComplexityClassId,
        visualization_controller::{layer_assignment::assign_layers, VisualizationController},
    };

    use super::*;

    fn get_arranged_graph() -> LayeredGraph<Vec<ComplexityClassId>, database::relation::RelationType>
    {
        let data = database::get_data();
        let graph = VisualizationController::generate_graph(&data);
        let condensated_graph = condensation(graph, true);
        assign_layers(condensated_graph)
    }

    #[test]
    fn preserves_nodes() {
        let lg = get_arranged_graph();
        let lg_dummy = lg.clone().add_dummy_nodes(vec![]);

        for node in lg.graph().node_indices() {
            assert!(
                lg_dummy
                    .graph()
                    .node_indices()
                    .find(|n| *n == node)
                    .is_some(),
                "Node with index {} is not preserved after adding dummy nodes",
                node.index()
            );
            let before = lg.graph().node_weight(node).unwrap();
            let after = lg_dummy.graph().node_weight(node).unwrap();
            assert_eq!(before, after, "The weight of node {} is different after adding dummy nodes (before: {:?}, after {:?})", node.index(), before, after);
        }
    }

    #[test]
    fn preserves_layermap() {
        let lg = get_arranged_graph();
        let lg_dummy = lg.clone().add_dummy_nodes(vec![]);

        for node in lg.graph().node_indices() {
            let layer_before = lg.layer_map().get(&node).unwrap();
            let layer_after = lg_dummy.layer_map().get(&node).unwrap();
            assert!(layer_before == layer_after, "The layer of node {} is not preserved after adding dummy nodes (before: {}, after {})", node.index(), layer_before, layer_after);
        }
    }

    #[test]
    fn preserves_layers() {
        let lg = get_arranged_graph();
        let lg_dummy = lg.clone().add_dummy_nodes(vec![]);

        for layer_idx in 0..lg.layers().len() {
            for node in &lg.layers()[layer_idx] {
                assert!(
                    lg_dummy.layers()[layer_idx].contains(node),
                    "The layer of node {} is not preserved after dummy nodes (was in layer {})",
                    node.index(),
                    layer_idx
                );
            }
        }
    }

    #[test]
    fn only_inserted_dummy_nodes() {
        let lg = get_arranged_graph();
        let lg_dummy = lg.clone().add_dummy_nodes(vec![]);

        for node in lg_dummy.graph().node_indices() {
            // Skip nodes that are already present in the original graph
            if lg.graph().node_indices().find(|n| *n == node).is_some() {
                continue;
            }

            assert!(
                lg_dummy.graph().node_weight(node).unwrap().is_empty(),
                "add_dummy_nodes added the node {}, which is not a dummy node (weight: {:?})",
                node.index(),
                lg_dummy.graph().node_weight(node).unwrap()
            );
        }
    }

    #[test]
    fn preserves_edges() {
        let lg = get_arranged_graph();
        let lg_dummy = lg.clone().add_dummy_nodes(vec![]);

        for edge in lg.graph().edge_indices() {
            if lg.is_long_edge(edge) {
                continue;
            }

            assert!(
                lg_dummy
                    .graph()
                    .edge_indices()
                    .find(|e| *e == edge)
                    .is_some(),
                "Edge with index {} is not preserved after adding dummy nodes",
                edge.index()
            );
            let before = lg.graph().edge_weight(edge).unwrap();
            let after = lg_dummy.graph().edge_weight(edge).unwrap();
            assert_eq!(before, after, "The weight of edge {} is different after adding dummy nodes (before: {:?}, after {:?})", edge.index(), before, after);

            let (b1, b2) = lg.graph().edge_endpoints(edge).unwrap();
            let (a1, a2) = lg_dummy.graph().edge_endpoints(edge).unwrap();

            assert_eq!(b1, a1, "After adding dummy nodes, the edge {} from node {} to node {} now points from node {} to node {}", edge.index(), b1.index(), b2.index(), a1.index(), a2.index());
            assert_eq!(b2, a2, "After adding dummy nodes, the edge {} from node {} to node {} now points from node {} to node {}", edge.index(), b1.index(), b2.index(), a1.index(), a2.index());
        }
    }

    #[test]
    fn no_reverse_edges() {
        let lg = get_arranged_graph();
        let lg_dummy = lg.clone().add_dummy_nodes(vec![]);

        for edge in lg_dummy.graph().edge_indices() {
            let (n1, n2) = lg_dummy.graph().edge_endpoints(edge).unwrap();

            let layer1 = lg_dummy.layer_map().get(&n1).unwrap();
            let layer2 = lg_dummy.layer_map().get(&n2).unwrap();

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
    fn only_short_edges() {
        let lg = get_arranged_graph();
        let lg_dummy = lg.clone().add_dummy_nodes(vec![]);

        for edge in lg_dummy.graph().edge_indices() {
            assert!(
                lg_dummy.is_short_edge(edge),
                "Edge {} is a long edge",
                edge.index()
            );
        }
    }

    #[test]
    fn long_edges_still_connected() {
        let lg = get_arranged_graph();
        let lg_dummy: LayeredGraph<Vec<ComplexityClassId>, database::relation::RelationType> =
            lg.clone().add_dummy_nodes(vec![]);

        for edge in lg.graph().edge_indices() {
            if lg.is_short_edge(edge) {
                continue;
            }
            let (from, to) = lg.graph().edge_endpoints(edge).unwrap();

            assert!(has_path_connecting(lg_dummy.graph(), from, to, None), "The edge {} from node {} to node {} is no longer connected after adding dummy nodes", edge.index(), from.index(), to.index());
        }
    }

    #[test]
    fn long_edges_connected_via_dummynodes() {
        let lg = get_arranged_graph();
        let lg_dummy: LayeredGraph<Vec<ComplexityClassId>, database::relation::RelationType> =
            lg.clone().add_dummy_nodes(vec![]);

        for edge in lg.graph().edge_indices() {
            if lg.is_short_edge(edge) {
                continue;
            }
            let (from, to) = lg.graph().edge_endpoints(edge).unwrap();

            // Create clone of graph that only contains "from", "to" and all dummy nodes
            let mut gr = lg_dummy.graph().clone();
            for node in lg_dummy.graph().node_indices() {
                // skip "from" and "to"
                if node == from || node == to {
                    continue;
                }
                // skip dummy nodes
                if lg_dummy.graph().node_weight(node).unwrap().is_empty() {
                    continue;
                }

                gr.remove_node(node);
            }

            assert!(has_path_connecting(&gr, from, to, None), "The edge {} from node {} to node {} is no longer connected after adding dummy nodes", edge.index(), from.index(), to.index());
        }
    }
}
