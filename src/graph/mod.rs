use egui::{Rect, Scene, Widget};
use node::NodeWidget;
use relation::RelationWidget;

use crate::{
    app::Selection,
    model::{
        relation::{Relation, Subset},
        Model,
    },
    visualization_controller::VisualizationController,
};

mod node;
mod relation;

pub struct GraphWidget<'a> {
    pub selected: &'a mut Selection,
    pub model: &'a Model,
    pub visualization_controller: &'a VisualizationController,
    pub scene_rect: &'a mut Rect,
}

impl Widget for GraphWidget<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let scene = Scene::new()
            .max_inner_size([10000.0, 10000.0])
            .zoom_range(egui::Rangef::new(0.0, 1.0));

        scene
            .show(ui, self.scene_rect, |ui| {
                for relation in self.model.relations() {
                    if let Some((from, to)) = match relation {
                        Relation::Subset(Subset { from, to }) => Some((from, to)),
                        Relation::Equal(Subset { from, to }, _) => Some((from, to)),
                        Relation::Unknown => None,
                    } {
                        let response = ui.add(RelationWidget {
                            path: self
                                .visualization_controller
                                .get_edge_path(*from, *to)
                                .unwrap(),
                            relation,
                            is_selected: match self.selected {
                                Selection::Relation((f, t)) => from == f && to == t,
                                _ => false,
                            },
                        });
                        if response.clicked() {
                            *self.selected = Selection::Relation((*from, *to));
                        }
                    }
                }
                for class in self.model.classes() {
                    let response = ui.put(
                        egui::Rect::from_center_size(
                            *self
                                .visualization_controller
                                .get_position(&class.id)
                                .unwrap(),
                            egui::vec2(100.0, 100.0),
                        ),
                        NodeWidget {
                            label: class.names.first().unwrap().clone(),
                            is_selected: match self.selected {
                                Selection::ComplexityClass(id) => *id == class.id,
                                _ => false,
                            },
                            tags: class.tags.clone(),
                        },
                    );
                    if response.clicked() {
                        *self.selected = Selection::ComplexityClass(class.id);
                    }
                }
            })
            .response
    }
}
