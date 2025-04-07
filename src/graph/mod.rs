use egui::{Rect, Scene, Widget};
use node::NodeWidget;
use relation::RelationWidget;

use crate::{model::{
    complexity_class::ComplexityClassId,
    relation::{Relation, Subset},
    Model,
}, visualization_controller::VisualizationController};

mod node;
mod relation;

pub struct GraphWidget<'a> {
    pub selected_class: &'a mut Option<ComplexityClassId>,
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
                        ui.add(RelationWidget {
                            from: *self.visualization_controller.get_position(from).unwrap(),
                            to: *self.visualization_controller.get_position(to).unwrap(),
                            relation,
                        });
                    }
                }
                for class in self.model.classes() {
                    let response = ui.put(
                        egui::Rect::from_center_size(
                            *self.visualization_controller.get_position(&class.id).unwrap(),
                            egui::vec2(100.0, 100.0),
                        ),
                        NodeWidget {
                            label: class.names.first().unwrap().clone(),
                            selected: self.selected_class.is_some_and(|c| c == class.id),
                            tags: class.tags.clone(),
                        },
                    );
                    if response.clicked() {
                        *self.selected_class = Some(class.id);
                    }
                }
            })
            .response
    }
}
