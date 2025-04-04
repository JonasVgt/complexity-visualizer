use std::{collections::HashMap, fmt::Debug};

use petgraph::graph::NodeIndex;

use super::layered_graph::LayeredGraph;

pub fn compute_horizontal_coordinate<N, E>(graph: &LayeredGraph<N, E>) -> HashMap<NodeIndex, f32>
where
    N: Debug,
{
    let mut pos: HashMap<NodeIndex, f32> = HashMap::new();

    // Assign coordinates to first layer
    for (i, n) in graph.layers()[0].iter().enumerate() {
        pos.insert(*n, i as f32);
    }

    for (layer_idx, layer) in graph.layers().iter().enumerate().skip(1) {
        // Assign node pos to average position of parents
        let mut prev_node = None;
        for node in layer {
            let mut sum = 0;
            let mut num = 0;
            for parent in graph.parents(*node) {
                if let Some(pos) = find_pos(graph.layers().get(layer_idx - 1).unwrap(), parent) {
                    sum += pos;
                    num += 1;
                }
            }
            let x_pos = match (prev_node, num) {
                (Some(p), 0) => *pos.get(p).unwrap() + 1.0,
                (None, 0) => 0.0,
                (Some(p), _) => f32::max(*pos.get(p).unwrap() + 1.0, 0.0),
                (None, _) => sum as f32 / num as f32,
            };

            pos.insert(*node, x_pos);
            prev_node = Some(node);
        }
    }
    pos
}

fn find_pos(layer: &[NodeIndex], node: NodeIndex) -> Option<usize> {
    for (i, n) in layer.iter().enumerate() {
        if *n == node {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use petgraph::algo::condensation;

    use crate::{
        database::{self, MyDatabase},
        visualization_controller::{
            layer_assignment::assign_layers, vertex_ordering::order_vertices,
            VisualizationController,
        },
    };

    use super::*;

    fn get_graph() -> LayeredGraph<Vec<u64>, database::relation::RelationType> {
        let data = MyDatabase::get_data();
        let vc = VisualizationController::new(&data);
        let graph = vc.graph;
        let condensated_graph = condensation(graph, true);
        let graph_with_dummy_nodes = assign_layers(condensated_graph).add_dummy_nodes(vec![]);
        order_vertices(graph_with_dummy_nodes)
    }

    #[test]
    fn all_nodes_assigned() {
        let lg = get_graph();
        let hor_coord = compute_horizontal_coordinate(&lg);

        for node in lg.graph().node_indices() {
            assert!(
                hor_coord.contains_key(&node),
                "There is no horizontal coordinate assigned to node {}",
                node.index()
            )
        }
    }

    #[test]
    fn does_not_change_vertex_ordering() {
        let lg = get_graph();
        let positions = compute_horizontal_coordinate(&lg);

        for layer in lg.layers() {
            let mut prev_pos = f32::MIN;
            for node in layer {
                let npos = *positions.get(node).unwrap();
                assert!(
                    npos > prev_pos,
                    "Position of node {} is smaller (or equal) to its neighbor",
                    node.index()
                );
                prev_pos = npos;
            }
        }
    }
}
