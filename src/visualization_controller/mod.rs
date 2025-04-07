mod dummy_nodes;
mod horizontal_coordinate;
mod layer_assignment;
mod layered_graph;
mod vertex_ordering;

use std::collections::HashMap;

use egui::Pos2;
use horizontal_coordinate::compute_horizontal_coordinate;
use layer_assignment::assign_layers;
use petgraph::{algo::condensation, graph::node_index, Graph};
use vertex_ordering::order_vertices;

use crate::model::{
        complexity_class::ComplexityClassId,
        relation::{Relation, Subset},
        Model,
    };

pub struct VisualizationController {
    positions: HashMap<ComplexityClassId, Pos2>,
}

impl VisualizationController {
    pub fn new() -> Self {
        Self {
            positions: HashMap::new(),
        }
    }

    fn generate_graph(model: &Model) -> Graph<ComplexityClassId, ()> {
        let mut graph: Graph<ComplexityClassId, ()> =
            Graph::with_capacity(model.classes().len(), model.relations().len());
        let node_indices: HashMap<ComplexityClassId, usize> = model
            .classes()
            .iter()
            .map(|class| class.id)
            .map(|id| (id, graph.add_node(id).index()))
            .collect();

        model.relations().iter().for_each(|relation| {
            let edges = match relation {
                Relation::Subset(Subset { from, to }) => vec![(from, to)],
                Relation::Equal(Subset { from, to }, _) => vec![(from, to), (to, from)],
                Relation::Unknown => vec![],
            };
            for (from, to) in edges {
                graph.add_edge(
                    node_index(*node_indices.get(from).unwrap()),
                    node_index(*node_indices.get(to).unwrap()),
                    (),
                );
            }
        });

        graph
    }

    /*
     * Arranges the nodes according to hierarchical drawing algorithms
     * See: Healy, Patrick; Nikolov, Nikola S. (2014), "Hierarchical Graph Drawing", p.409-453
     * https://cs.brown.edu/people/rtamassi/gdhandbook/
     */
    pub fn arrange(&mut self, model: &Model) {
        let graph = Self::generate_graph(model);

        // An Directed Acyclic Graph containing the complexity classes. Equal classes are stored in a single node
        let condensated_graph = condensation(graph, true);
        let layered_graph = assign_layers(condensated_graph);

        let mut graph_with_dummynodes = layered_graph.add_dummy_nodes(vec![]);
        graph_with_dummynodes = order_vertices(graph_with_dummynodes);

        let mut map: HashMap<ComplexityClassId, Pos2> = HashMap::new();

        let hor_coordinates = compute_horizontal_coordinate(&graph_with_dummynodes);
        let mut x = 0;
        for layer in graph_with_dummynodes.layers().clone() {
            for node in layer {
                if graph_with_dummynodes
                    .graph()
                    .node_weight(node)
                    .unwrap_or(&vec![])
                    .is_empty()
                {
                    continue;
                }

                let y = *hor_coordinates.get(&node).unwrap();
                let classes = graph_with_dummynodes.graph().node_weight(node).unwrap();

                for i in 0..classes.len() {
                    let cy = y - (classes.len() as f32 / 2.0) + i as f32;
                    let pos = Pos2::new(x as f32 * 100.0, cy * 100.0);
                    map.insert(*classes.get(i).unwrap(), pos);
                }
            }
            x += 1;
        }

        self.positions = map;
    }

    pub fn get_position(&self, id: &ComplexityClassId) -> Option<&Pos2> {
        self.positions.get(id)
    }
}
