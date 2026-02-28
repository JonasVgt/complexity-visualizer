mod dummy_nodes;
mod horizontal_coordinate;
mod layer_assignment;
mod layered_graph;
mod vertex_ordering;

use std::{collections::HashMap, vec};

use egui::{pos2, Pos2};
use horizontal_coordinate::compute_horizontal_coordinate;
use layer_assignment::assign_layers;
use petgraph::{
    algo::{all_simple_paths, condensation},
    graph::{node_index, NodeIndex},
    Graph,
};
use vertex_ordering::order_vertices;

use crate::model::{
    complexity_class::ComplexityClassId,
    relation::{RelationComposition, RelationCompositionId},
    Model,
};

pub struct VisualizationController {
    positions: HashMap<ComplexityClassId, Pos2>,
    edge_paths: HashMap<RelationCompositionId, Vec<Pos2>>,
    node_spacing: f32,
    #[cfg(debug_assertions)]
    debug_layers: HashMap<ComplexityClassId, usize>,
    #[cfg(debug_assertions)]
    debug_sort_index: HashMap<ComplexityClassId, usize>,
    #[cfg(debug_assertions)]
    debug_dummy_node_positions: Vec<Pos2>,
}

impl VisualizationController {
    pub fn new() -> Self {
        #[cfg(not(debug_assertions))]
        return Self {
            positions: HashMap::new(),
            edge_paths: HashMap::new(),
            node_spacing: 150.0,
        };

        #[cfg(debug_assertions)]
        return Self {
            positions: HashMap::new(),
            edge_paths: HashMap::new(),
            node_spacing: 150.0,
            debug_layers: HashMap::new(),
            debug_sort_index: HashMap::new(),
            debug_dummy_node_positions: Vec::new(),
        };
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

        model.relation_compositions().iter().for_each(|relation| {
            let edges = match &relation {
                RelationComposition::Subset(_) => vec![(relation.get_from(), relation.get_to())],
                RelationComposition::Equalily(_) => vec![
                    (relation.get_from(), relation.get_to()),
                    (relation.get_to(), relation.get_from()),
                ],
            };
            for (from, to) in edges {
                graph.add_edge(
                    node_index(*node_indices.get(&from).unwrap()),
                    node_index(*node_indices.get(&to).unwrap()),
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

        #[cfg(debug_assertions)]
        {
            self.debug_layers = layered_graph
                .layer_map()
                .iter()
                .map(|(node, layer)| (layered_graph.graph().node_weight(*node).unwrap(), layer))
                .flat_map(|(classes, layer)| classes.iter().map(|c| (*c, *layer)))
                .collect();
        }

        let mut graph_with_dummynodes = layered_graph.add_dummy_nodes(vec![]);
        graph_with_dummynodes = order_vertices(graph_with_dummynodes);

        #[cfg(debug_assertions)]
        {
            for layer in graph_with_dummynodes.layers() {
                for (i, node) in layer.iter().enumerate() {
                    let classes = graph_with_dummynodes.graph().node_weight(*node).unwrap();
                    for class in classes {
                        self.debug_sort_index.insert(*class, i);
                    }
                }
            }
        }

        let node_positions: HashMap<NodeIndex, Pos2> =
            compute_horizontal_coordinate(&graph_with_dummynodes)
                .into_iter()
                .map(|(node, y)| {
                    (
                        node,
                        pos2(graph_with_dummynodes.get_layer(node).unwrap() as f32, y),
                    )
                })
                .collect();

        self.positions = HashMap::new();

        for layer in graph_with_dummynodes.layers().clone() {
            for node in layer {
                if graph_with_dummynodes
                    .graph()
                    .node_weight(node)
                    .unwrap_or(&vec![])
                    .is_empty()
                {
                    #[cfg(debug_assertions)]
                    {
                        let pos = *node_positions.get(&node).unwrap();
                        self.debug_dummy_node_positions.push(pos);
                    }

                    continue;
                }
                let p = *node_positions.get(&node).unwrap();
                let classes = graph_with_dummynodes.graph().node_weight(node).unwrap();

                for i in 0..classes.len() {
                    let y = p.y - (classes.len() as f32 / 2.0) + i as f32;
                    let pos = Pos2::new(p.x as f32, y);
                    self.positions.insert(*classes.get(i).unwrap(), pos);
                }
            }
        }
        // Assign paths for relations
        self.edge_paths = HashMap::new();
        for relation in model.relation_compositions() {
            let edges = match &relation {
                RelationComposition::Subset(_) => vec![(relation.get_from(), relation.get_to())],
                RelationComposition::Equalily(_) => vec![
                    (relation.get_from(), relation.get_to()),
                    (relation.get_to(), relation.get_from()),
                ],
            };

            for (from, to) in &edges {
                let from_node = graph_with_dummynodes
                    .graph()
                    .node_indices()
                    .find(|n| {
                        graph_with_dummynodes
                            .graph()
                            .node_weight(*n)
                            .unwrap()
                            .contains(from)
                    })
                    .unwrap();
                let to_node = graph_with_dummynodes
                    .graph()
                    .node_indices()
                    .find(|n| {
                        graph_with_dummynodes
                            .graph()
                            .node_weight(*n)
                            .unwrap()
                            .contains(to)
                    })
                    .unwrap();

                if from_node == to_node {
                    self.edge_paths.insert(
                        relation.id(),
                        vec![
                            *self.positions.get(from).unwrap(),
                            *self.positions.get(to).unwrap(),
                        ],
                    );
                    continue;
                }

                // Create clone of graph that only contains "from", "to" and all dummy nodes
                let mut gr = graph_with_dummynodes.graph().clone();
                for edge in graph_with_dummynodes.graph().edge_indices() {
                    let (f, t) = graph_with_dummynodes.graph().edge_endpoints(edge).unwrap();

                    if graph_with_dummynodes
                        .graph()
                        .node_weight(f)
                        .unwrap()
                        .is_empty()
                        && graph_with_dummynodes
                            .graph()
                            .node_weight(t)
                            .unwrap()
                            .is_empty()
                    {
                        continue;
                    }

                    if f == from_node && t == to_node {
                        continue;
                    }

                    if f == from_node
                        && graph_with_dummynodes
                            .graph()
                            .node_weight(t)
                            .unwrap()
                            .is_empty()
                    {
                        continue;
                    }

                    if graph_with_dummynodes
                        .graph()
                        .node_weight(f)
                        .unwrap()
                        .is_empty()
                        && t == to_node
                    {
                        continue;
                    }

                    gr.remove_edge(edge);
                }
                let path: Vec<NodeIndex> = all_simple_paths(&gr, from_node, to_node, 0, None)
                    .next()
                    .unwrap();
                let mut path_pos: Vec<Pos2> = path
                    .into_iter()
                    .map(|n| *node_positions.get(&n).unwrap())
                    .collect();
                path_pos[0] = *self.positions.get(from).unwrap();
                let len = path_pos.len();
                path_pos[len - 1] = *self.positions.get(to).unwrap();

                self.edge_paths.insert(relation.id(), path_pos);
            }
        }
    }

    pub fn get_position(&self, id: &ComplexityClassId) -> Option<Pos2> {
        self.positions.get(id).map(|n| *n * self.node_spacing)
    }

    pub fn get_edge_path(&self, id: RelationCompositionId) -> Option<Vec<Pos2>> {
        self.edge_paths
            .get(&id)
            .map(|v| v.iter().map(|p| *p * self.node_spacing).collect())
    }

    #[cfg(debug_assertions)]
    pub fn get_node_layer(&self, id: &ComplexityClassId) -> Option<usize> {
        self.debug_layers.get(id).map(|l| *l)
    }

    #[cfg(debug_assertions)]
    pub fn get_node_sort_idx(&self, id: &ComplexityClassId) -> Option<usize> {
        self.debug_sort_index.get(id).map(|l| *l)
    }

    #[cfg(debug_assertions)]
    pub fn get_dummy_node_postions(&self) -> Vec<Pos2> {
        self.debug_dummy_node_positions
            .iter()
            .map(|p| *p * self.node_spacing)
            .collect()
    }
}
