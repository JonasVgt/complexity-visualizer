use egui::{Rect, Scene, Widget};
use node::NodeWidget;
use relation::RelationWidget;

use crate::model::{complexity_class::ComplexityClass, relation::Relation, Model};

mod node;
mod relation;

pub struct GraphWidget<'a> {
    pub selected_class: &'a mut Option<u64>,
    pub model: &'a Model,
    pub scene_rect: &'a mut Rect,
}

impl Widget for GraphWidget<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::Frame::group(ui.style())
            .inner_margin(0.0)
            .show(ui, |ui| {
                let scene = Scene::new()
                    .max_inner_size([10000.0, 10000.0])
                    .zoom_range(egui::Rangef::new(0.0, 1.0));

                scene
                    .show(ui, self.scene_rect, |ui| {
                        for relation in self.model.relations() {
                            if let Relation::Subset { from, to } = relation {
                                ui.add(RelationWidget {
                                    from: self
                                        .model
                                        .get_position(&ComplexityClass::hash_id(&from))
                                        .unwrap()
                                        .clone(),
                                    to: self
                                        .model
                                        .get_position(&ComplexityClass::hash_id(&to))
                                        .unwrap()
                                        .clone(),
                                    relation,
                                });
                            }
                        }
                        for class in self.model.classes() {
                            let response = ui.put(
                                egui::Rect::from_center_size(
                                    self.model
                                        .get_position(&class.calculate_id_hash())
                                        .unwrap()
                                        .clone(),
                                    egui::vec2(100.0, 100.0),
                                ),
                                NodeWidget {
                                    label: class.names.first().unwrap().clone(),
                                    selected: self
                                        .selected_class
                                        .is_some_and(|c| c == class.calculate_id_hash()),
                                },
                            );
                            if response.clicked() {
                                *self.selected_class = Some(class.calculate_id_hash());
                            }
                        }
                    })
                    .response
            })
            .response
    }
}
