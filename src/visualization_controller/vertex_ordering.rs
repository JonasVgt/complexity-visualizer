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
        let mut i = 0;
        for parent in parent_level {
            if neighbors.contains(&parent) {
                sum += i;
                num += 1;
            }
            i += 1;
        }
        return (10000.0 * (sum as f32 / num as f32)) as i32;
    };

    let mut layers = graph.layers().clone();
    for i in 1..graph.layers().len() {
        let (done, unsorted) = layers.split_at_mut(i);
        unsorted[0].sort_by_key(|node| heur(node.clone(), done.last().unwrap()));
    }
    return LayeredGraph::new(graph.graph().clone(), layers);
}
