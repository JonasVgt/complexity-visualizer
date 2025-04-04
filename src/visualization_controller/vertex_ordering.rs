use petgraph::graph::NodeIndex;

use super::layered_graph::LayeredGraph;

pub fn order_vertices<N, E>(graph: LayeredGraph<N, E>) -> LayeredGraph<N, E>
where
    N: Clone,
    E: Clone,
{
    let heur = |node: NodeIndex, parent_level: &Vec<NodeIndex>| {
        let mut sum = 0;
        let mut num = 0;
        let neighbors: Vec<NodeIndex> = graph
            .graph()
            .neighbors_directed(node, petgraph::Direction::Incoming)
            .collect();
        for (i, parent) in parent_level.iter().enumerate() {
            if neighbors.contains(parent) {
                sum += i;
                num += 1;
            }
        }
        (10000.0 * (sum as f32 / num as f32)) as i32
    };

    let mut layers = graph.layers().clone();
    for i in 1..graph.layers().len() {
        let (done, unsorted) = layers.split_at_mut(i);
        unsorted[0].sort_by_key(|node| heur(*node, done.last().unwrap()));
    }
    LayeredGraph::with_layer_vec(graph.into_graph(), layers)
}

#[cfg(test)]
mod tests {
    use petgraph::algo::condensation;

    use crate::{
        database::{self, MyDatabase},
        visualization_controller::{layer_assignment::assign_layers, VisualizationController},
    };

    use super::*;

    fn get_graph() -> LayeredGraph<Vec<u64>, database::relation::RelationType> {
        let data = MyDatabase::get_data();
        let vc = VisualizationController::new(&data);
        let graph = vc.graph;
        let condensated_graph = condensation(graph, true);
        let graph_with_dummy_nodes = assign_layers(condensated_graph).add_dummy_nodes(vec![]);
        graph_with_dummy_nodes
    }

    #[test]
    fn preserves_graph() {
        let lg = get_graph();
        let lg_ordered = order_vertices(lg.clone());

        let a_ns = lg.graph().raw_nodes().iter().map(|n| &n.weight);
        let b_ns = lg_ordered.graph().raw_nodes().iter().map(|n| &n.weight);

        assert!(
            a_ns.eq(b_ns),
            "Nodes of the graph are not preserved after ordering"
        );

        let a_es = lg
            .graph()
            .raw_edges()
            .iter()
            .map(|e| (e.source(), e.target(), &e.weight));
        let b_es = lg_ordered
            .graph()
            .raw_edges()
            .iter()
            .map(|e| (e.source(), e.target(), &e.weight));

        assert!(
            a_es.eq(b_es),
            "Edges of the graph are not preserved after ordering"
        );
    }

    #[test]
    fn preserves_layermap() {
        let lg = get_graph();
        let lg_ordered = order_vertices(lg.clone());

        for node in lg.graph().node_indices() {
            let layer_before = lg.layer_map().get(&node).unwrap();
            let layer_after = lg_ordered.layer_map().get(&node).unwrap();
            assert!(
                layer_before == layer_after,
                "The layer of node {} is not preserved after ordering (before: {}, after {})",
                node.index(),
                layer_before,
                layer_after
            );
        }
    }

    #[test]
    fn preserves_layers() {
        let lg = get_graph();
        let lg_ordered = order_vertices(lg.clone());

        for layer_idx in 0..lg.layers().len() {
            for node in &lg.layers()[layer_idx] {
                assert!(
                    lg_ordered.layers()[layer_idx].contains(node),
                    "The layer of node {} is not preserved after dummy nodes (was in layer {})",
                    node.index(),
                    layer_idx
                );
            }
        }
    }

    #[test]
    fn no_duplicate_nodes_in_layers() {
        let lg = get_graph();
        let lg_ordered = order_vertices(lg.clone());

        for node in lg_ordered.graph().node_indices() {
            let num_occ = lg_ordered
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
}
