use petgraph::{graph::NodeIndex, Graph};

pub struct LayeredGraph<N, E> {
    pub graph: Graph<N, E>,
    pub layers: Vec<Vec<NodeIndex>>,
}

impl<N, E> LayeredGraph<N, E> {
    pub fn new(graph: Graph<N, E>, layers: Vec<Vec<NodeIndex>>) -> Self {
        Self { graph, layers }
    }
}
