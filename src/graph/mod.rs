use egui::{Rect, Scene, Widget};
use node::NodeWidget;
use relation::RelationWidget;

use crate::model::Model;

mod node;
mod relation;

pub struct GraphWidget<'a> {
    pub selected_class: &'a mut Option<u32>,
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
                    .zoom_range(egui::Rangef::new(0.0, 100.00));

                scene
                    .show(ui, self.scene_rect, |ui| {
                        for class in self.model.classes() {
                            let response = ui.put(
                                egui::Rect::from_center_size(
                                    self.model.get_position(&class.id).unwrap().clone(),
                                    egui::vec2(50.0, 50.0),
                                ),
                                NodeWidget {
                                    label: class.name.clone(),
                                },
                            );
                            if response.clicked() {
                                *self.selected_class = Some(class.id);
                            }
                        }
                        for relation in self.model.relations() {
                            ui.add(RelationWidget {
                                from: self
                                    .model
                                    .get_position(&relation.id_subset)
                                    .unwrap()
                                    .clone(),
                                to: self
                                    .model
                                    .get_position(&relation.id_superset)
                                    .unwrap()
                                    .clone(),
                            });
                        }
                    })
                    .response
            })
            .response
    }
}
