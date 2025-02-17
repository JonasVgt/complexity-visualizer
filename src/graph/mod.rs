use egui::{pos2, Rect, Scene, Widget};
use node::NodeWidget;
use relation::RelationWidget;

use crate::model::Model;

mod node;
mod relation;

pub struct GraphWidget<'a> {
    pub selected_class: &'a mut Option<u32>,
    pub model: &'a Model,
}

impl Widget for GraphWidget<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let response = Scene::new().show(
            ui,
            &mut Rect::from_min_size(
                pos2(0.0, 0.0),
                egui::Vec2 {
                    x: 1000.0,
                    y: 1000.0,
                },
            ),
            |ui| {
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
            },
        );
        return response.response;
    }
}
