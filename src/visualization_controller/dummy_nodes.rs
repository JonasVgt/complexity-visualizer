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
        return res;
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
            let from_layer = self.layer_map().get(&from).unwrap().clone();
            let to_layer = self.layer_map().get(&to).unwrap().clone();

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
        return self;
    }
}
