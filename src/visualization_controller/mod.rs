mod dummy_nodes;
mod horizontal_coordinate;
mod layer_assignment;

use std::collections::HashMap;

use dummy_nodes::add_dummy_nodes;
use egui::Pos2;
use horizontal_coordinate::compute_horizontal_coordinate;
use layer_assignment::assign_layers;
use petgraph::{
    algo::condensation,
    graph::{node_index, NodeIndex},
    Directed, Graph,
};

use crate::database::{data::Data, relation::RelationType};

pub struct VisualizationController {
    graph: Graph<u64, RelationType, Directed>,
}

impl<'a> VisualizationController {
    pub fn new(data: &'a Data) -> Self {
        let mut graph: Graph<u64, RelationType> =
            Graph::with_capacity(data.classes.len(), data.relations.len());
        let node_indices: HashMap<u64, usize> = data
            .classes
            .iter()
            .map(|class| class.calculate_id_hash())
            .map(|id| (id, graph.add_node(id).index()))
            .collect();

        data.relations.iter().for_each(|relation| {
            graph.add_edge(
                node_index(
                    node_indices
                        .get(&relation.calculate_from_hash())
                        .unwrap()
                        .clone(),
                ),
                node_index(
                    node_indices
                        .get(&relation.calculate_to_hash())
                        .unwrap()
                        .clone(),
                ),
                relation.relation_type,
            );
        });

        Self { graph }
    }

    /*
     * Arranges the nodes according to hierarchical drawing algorithms
     * See: Healy, Patrick; Nikolov, Nikola S. (2014), "Hierarchical Graph Drawing", p.409-453
     * https://cs.brown.edu/people/rtamassi/gdhandbook/
     */
    pub fn arrange(self) -> HashMap<u64, Pos2> {
        // An Directed Acyclic Graph containting the complexity classes. Equal classes are stored in a single node
        let condensated_graph = condensation(self.graph, true);

        let layers: Vec<Vec<NodeIndex>> = assign_layers(&condensated_graph);

        let (graph_with_dummynodes, mut layers) =
            add_dummy_nodes(condensated_graph, layers, vec![]);

        let heur = |node: NodeIndex, parent_level: &Vec<NodeIndex>| {
            let mut sum = 0;
            let mut num = 0;
            let neighbors: Vec<NodeIndex> = graph_with_dummynodes
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

        let mut map: HashMap<u64, Pos2> = HashMap::new();

        let hor_coordinates = compute_horizontal_coordinate(&graph_with_dummynodes, &layers);
        let mut x = 0;
        for level in layers {
            for node in level {
                if graph_with_dummynodes
                    .node_weight(node)
                    .unwrap_or(&vec![])
                    .is_empty()
                {
                    continue;
                }

                let y = hor_coordinates.get(&node).unwrap().clone();
                let classes = graph_with_dummynodes.node_weight(node).unwrap();

                for i in 0..classes.len() {
                    let cy = y - (classes.len() as f32 / 2.0) + i as f32;
                    let pos = Pos2::new(x as f32 * 100.0, cy * 100.0);
                    map.insert(classes.get(i).unwrap().clone(), pos);
                }
            }
            x += 1;
        }

        return map;
    }
}
