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
