use std::usize;

use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    Graph,
};

pub struct LayeredGraph<N, E> {
    graph: Graph<N, E>,
    pub layers: Vec<Vec<NodeIndex>>,
}

impl<N, E> LayeredGraph<N, E> {
    pub fn new(graph: Graph<N, E>, layers: Vec<Vec<NodeIndex>>) -> Self {
        Self { graph, layers }
    }

    pub fn graph(&self) -> &Graph<N, E> {
        &self.graph
    }

    pub fn add_node(&mut self, weight: N, layer: usize) -> NodeIndex {
        let idx = self.graph.add_node(weight);
        self.layers[layer].push(idx);
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
