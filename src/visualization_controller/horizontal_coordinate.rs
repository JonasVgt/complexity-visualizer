use std::collections::HashMap;

use petgraph::{graph::NodeIndex, Graph};

pub fn compute_horizontal_coordinate<N, E>(
    graph: &Graph<N, E>,
    layers: &Vec<Vec<NodeIndex>>,
) -> HashMap<NodeIndex, f32> {
    let mut pos = HashMap::new();
    for i in 0..layers.get(0).unwrap().len() {
        pos.insert(layers.get(0).unwrap().get(i).unwrap().clone(), i as f32);
    }
    for layer_idx in 1..layers.len() {
        let layer = layers.get(layer_idx).unwrap();
        for node in layer {
            let neighbors = graph.neighbors_directed(*node, petgraph::Direction::Outgoing);
            let mut sum = 0;
            let mut num = 0;
            for neighbor in neighbors {
                if let Some(pos) = find_pos(layers.get(layer_idx - 1).unwrap(), neighbor) {
                    sum += pos;
                    num += 1;
                }
            }
            pos.insert(node.clone(), sum as f32 / num as f32);
        }
        for node in layer {
            if pos.get(node).unwrap().is_nan() {
                let p = find_pos(layer, node.clone()).unwrap();
                let left_pos = if p >= 1 {pos.get(layer.get(p-1).unwrap()).unwrap().clone()} else {0.0};
                let right_pos = if p+1 < layer.len() {pos.get(layer.get(p+1).unwrap()).unwrap().clone()} else {left_pos+2.0};

                pos.insert(node.clone(), 0.5 * left_pos + 0.5 * right_pos);
            }
        }
    }
    return pos;
}

fn find_pos(layer: &Vec<NodeIndex>, node: NodeIndex) -> Option<usize> {
    let mut i = 0;
    for n in layer {
        if *n == node {
            return Some(i);
        }
        i += 1;
    }
    return None;
}