use petgraph::graph::NodeIndex;

use super::layered_graph::LayeredGraph;

fn compute_barycenter<N, E>(
    graph: &petgraph::Graph<N, E>,
    node: NodeIndex,
    parent_level: &[NodeIndex],
    direction: petgraph::Direction,
) -> f32 {
    let neighbors: Vec<NodeIndex> = graph.neighbors_directed(node, direction).collect();

    let num = neighbors.len();
    let sum: usize = parent_level
        .iter()
        .enumerate()
        .filter(|(_, p)| neighbors.contains(p))
        .map(|(i, _)| i)
        .sum();

    sum as f32 / num as f32
}

pub fn order_vertices<N, E>(graph: LayeredGraph<N, E>) -> LayeredGraph<N, E>
where
    N: Clone,
    E: Clone,
{
    let (graph, mut layers) = graph.into_graph_and_layers();

    loop {
        let mut sorted = vec![layers[0].clone()];
        for layer in layers.iter().skip(1) {
            let mut barycenter: Vec<(NodeIndex, f32)> = layer
                .clone()
                .into_iter()
                .map(|n| {
                    (
                        n,
                        compute_barycenter(
                            &graph,
                            n,
                            sorted.last().unwrap(),
                            petgraph::Direction::Incoming,
                        ),
                    )
                })
                .collect();
            barycenter.sort_by(|(_, a), (_, b)| f32::total_cmp(a, b));
            sorted.push(barycenter.into_iter().map(|(n, _)| n).collect())
        }

        let mut sorted2 = vec![sorted.last().unwrap().clone()];
        for layer in layers.iter().rev().skip(1) {
            let mut barycenter: Vec<(NodeIndex, f32)> = layer
                .clone()
                .into_iter()
                .map(|n| {
                    (
                        n,
                        compute_barycenter(
                            &graph,
                            n,
                            sorted2.last().unwrap(),
                            petgraph::Direction::Outgoing,
                        ),
                    )
                })
                .collect();
            barycenter.sort_by(|(_, a), (_, b)| f32::total_cmp(a, b));
            sorted2.push(barycenter.into_iter().map(|(n, _)| n).collect())
        }
        sorted2.reverse();
        if sorted2 == layers {
            break;
        } else {
            layers = sorted2;
        }
    }

    LayeredGraph::with_layer_vec(graph, layers)
}

#[cfg(test)]
mod tests {
    use petgraph::algo::condensation;

    use crate::{
        database::{self},
        model::complexity_class::ComplexityClassId,
        visualization_controller::{layer_assignment::assign_layers, VisualizationController},
    };

    use super::*;

    fn get_graph() -> LayeredGraph<Vec<ComplexityClassId>, database::relation::RelationType> {
        let data = database::get_data();
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
