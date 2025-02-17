use std::{cmp::Ordering, collections::HashMap};

use egui::{pos2, vec2, Pos2};
use petgraph::{
    algo::{has_path_connecting, tarjan_scc},
    graph::node_index,
    Directed, Graph,
};

use crate::database::data::Data;

pub struct VisualizationController {
    graph: Graph<u32, String, Directed>,
}

impl<'a> VisualizationController {
    pub fn new(data: &'a Data) -> Self {
        let mut graph: Graph<u32, String> =
            Graph::with_capacity(data.classes.len(), data.relations.len());
        let node_indices: HashMap<u32, usize> = data
            .classes
            .iter()
            .map(|class| class.id)
            .map(|id| (id, graph.add_node(id).index()))
            .collect();

        data.relations.iter().for_each(|relation| {
            graph.add_edge(
                node_index(node_indices.get(&relation.id_subset).unwrap().clone()),
                node_index(node_indices.get(&relation.id_superset).unwrap().clone()),
                relation.relation_type.clone(),
            );
        });

        Self { graph }
    }
    pub fn arrange(&self) -> HashMap<u32, Pos2> {
        let mut levels = tarjan_scc(&self.graph);
        levels.sort_unstable_by(|nodes1, nodes2| {
            let node1 = nodes1.get(0).unwrap();
            let node2 = nodes2.get(0).unwrap();
            if has_path_connecting(&self.graph, node1.clone(), node2.clone(), None) {
                Ordering::Greater
            } else if has_path_connecting(&self.graph, node1.clone(), node2.clone(), None) {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        let mut map: HashMap<u32, Pos2> = HashMap::new();

        let mut pos = pos2(0.0, 0.0);
        for level in levels {
            pos = pos2(pos.x + 100.0, 0.0);

            for node in level {
                let id = self.graph.node_weight(node).unwrap();

                map.insert(id.clone(), pos);
                pos += vec2(0.0, 100.0);
            }
        }
        return map;
    }
}
