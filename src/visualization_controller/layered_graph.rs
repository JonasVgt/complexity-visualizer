use std::{collections::HashMap, usize};

use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    Graph,
};

#[derive(Clone)]
pub struct LayeredGraph<N, E> {
    graph: Graph<N, E>,
    layers: Vec<Vec<NodeIndex>>,
    layer_map: HashMap<NodeIndex, usize>,
}

impl<N, E> LayeredGraph<N, E> {
    pub fn with_layer_vec(graph: Graph<N, E>, layers: Vec<Vec<NodeIndex>>) -> Self {
        let layer_map: HashMap<NodeIndex, usize> = layers
            .iter()
            .enumerate()
            .flat_map(|(id, layer)| layer.into_iter().map(move |x| (x.clone(), id)))
            .collect();

        Self {
            graph,
            layers,
            layer_map,
        }
    }

    pub fn with_layer_map(graph: Graph<N, E>, mut layer_map: HashMap<NodeIndex, usize>) -> Self {
        let min_layer = layer_map.values().min().map_or(0, |u| u.clone());
        let max_layer = layer_map.values().max().map_or(0, |u| u.clone());

        // remove empty layers in the beginning from layer_map
        if min_layer > 0 {
            layer_map = layer_map
                .into_iter()
                .map(|(n, l)| (n, l - min_layer))
                .collect();
        }

        // Convert layer_map to layers
        let layers: Vec<Vec<NodeIndex>> = layer_map.iter().fold(
            vec![vec![]; max_layer - min_layer + 1],
            |mut accu, (node, level)| {
                accu[*level].push(*node);
                accu
            },
        );

        Self {
            graph,
            layers,
            layer_map,
        }
    }

    pub fn graph(&self) -> &Graph<N, E> {
        &self.graph
    }

    pub fn into_graph(self) -> Graph<N, E> {
        self.graph
    }

    pub fn layers(&self) -> &Vec<Vec<NodeIndex>> {
        &self.layers
    }

    pub fn layer_map(&self) -> &HashMap<NodeIndex, usize> {
        &self.layer_map
    }

    pub fn parents(&self, node: NodeIndex) -> Vec<NodeIndex> {
        if let Some(layer) = self.get_layer(node) {
            self.graph
                .neighbors_directed(node, petgraph::Direction::Incoming)
                .filter(|n| self.in_layer(*n, layer - 1))
                .collect()
        } else {
            return vec![];
        }
    }

    pub fn in_layer(&self, node: NodeIndex, layer: usize) -> bool {
        self.get_layer(node).is_some_and(|l| l == layer)
    }

    pub fn get_layer(&self, node: NodeIndex) -> Option<usize> {
        self.layer_map.get(&node).copied()
    }

    #[cfg(test)]
    pub fn is_long_edge(&self, edge: EdgeIndex) -> bool {
        self.graph
            .edge_endpoints(edge)
            .map_or(false, |(n1, n2)| self.get_layer(n1).unwrap() + 1 < self.get_layer(n2).unwrap())
    }

    #[cfg(test)]
    pub fn is_short_edge(&self, edge: EdgeIndex) -> bool {
        self.graph
            .edge_endpoints(edge)
            .map_or(false, |(n1, n2)| self.get_layer(n1).unwrap() + 1 == self.get_layer(n2).unwrap())
    }

    pub fn add_node(&mut self, weight: N, layer: usize) -> NodeIndex {
        let idx = self.graph.add_node(weight);
        self.layers[layer].push(idx);
        self.layer_map.insert(idx, layer);
        return idx;
    }

    pub fn add_edge(
        &mut self,
        a: NodeIndex,
        b: NodeIndex,
        weight: E,
    ) -> petgraph::prelude::EdgeIndex {
        self.graph.add_edge(a, b, weight)
    }

    pub fn remove_edge(&mut self, e: EdgeIndex) -> Option<E> {
        self.graph.remove_edge(e)
    }
}
