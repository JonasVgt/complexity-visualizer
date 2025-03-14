use petgraph::{graph::NodeIndex, Graph};

pub fn order_vertices<N, E>(graph: &Graph<N, E>, mut layers: Vec<Vec<NodeIndex>>) -> Vec<Vec<NodeIndex>> {
    let heur = |node: NodeIndex, parent_level: &Vec<NodeIndex>| {
        let mut sum = 0;
        let mut num = 0;
        let neighbors: Vec<NodeIndex> = graph
            .neighbors_directed(node, petgraph::Direction::Outgoing)
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

    for i in 1..layers.len() {
        let (done, unsorted) = layers.split_at_mut(i);
        unsorted[0].sort_by_key(|node| heur(node.clone(), done.last().unwrap()));
    }
    return layers
}
