use std::collections::HashMap;

use petgraph::{graph::NodeIndex, Graph};

trait DummyNodes<N, T> {
    fn insert_dummy_nodes(
        &mut self,
        from: NodeIndex,
        to: NodeIndex,
        num: usize,
        node_weight: N,
        edge_weight: T,
    ) -> Vec<NodeIndex>
    where
        N: Clone,
        T: Clone;
}

impl<N, T> DummyNodes<N, T> for Graph<N, T> {
    fn insert_dummy_nodes(
        &mut self,
        from: NodeIndex,
        to: NodeIndex,
        num: usize,
        node_weight: N,
        edge_weight: T,
    ) -> Vec<NodeIndex>
    where
        N: Clone,
        T: Clone,
    {
        let mut res = vec![];
        // Remove existing edge or return, if it does not exist
        if let Some(e) = self.find_edge(from, to) {
            self.remove_edge(e);
        } else {
            println!("ERROR, no edge");
            return vec![];
        }

        // Add dummy nodes and edges
        let mut prev = from;
        for _ in 0..num {
            let curr = self.add_node(node_weight.clone());
            res.push(curr);
            self.add_edge(prev, curr, edge_weight.clone());
            prev = curr;
        }
        self.add_edge(prev, to, edge_weight.clone());
        return res;
    }
}

pub fn add_dummy_nodes<N, E>(
    mut graph: Graph<N, E>,
    layers: Vec<Vec<NodeIndex>>,
    dummy_node_weight: N,
) -> (Graph<N, E>, Vec<Vec<NodeIndex>>)
where
    E: Clone,
    N: Clone,
{
    let mut layer_map: HashMap<NodeIndex, usize> = layers
        .into_iter()
        .enumerate()
        .flat_map(|(id, layer)| layer.into_iter().map(move |x| (x, id)))
        .collect();

    for edge in graph.edge_indices() {
        let (from, to) = graph.edge_endpoints(edge).unwrap();

        let from_layer = layer_map.get(&from).unwrap().clone();
        let to_layer = layer_map.get(&to).unwrap().clone();

        if to_layer <= from_layer + 1 {
            continue;
        }
        let num_dummynodes = to_layer - from_layer - 1;

        let edge_weight = graph.edge_weight(edge).unwrap();

        let mut dummynodes = graph.insert_dummy_nodes(
            from,
            to,
            num_dummynodes,
            dummy_node_weight.clone(),
            edge_weight.clone(),
        );

        for i in 0..dummynodes.len() {
            layer_map.insert(dummynodes.remove(0), from_layer + 1 + i);
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
    return (graph, layers);
}
