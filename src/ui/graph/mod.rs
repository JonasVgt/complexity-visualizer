use egui::{Rect, Scene, Widget};
use node::NodeWidget;
use relation::RelationWidget;

use crate::{
    app::Selection, model::Model, ui::AppState, visualization_controller::VisualizationController,
};

mod node;
mod relation;

pub struct GraphWidget<'a> {
    pub selected: &'a mut Selection,
    pub model: &'a Model,
    pub visualization_controller: &'a VisualizationController,
    pub scene_rect: &'a mut Rect,
    #[allow(dead_code)]
    pub app_state: &'a AppState,
}

impl Widget for GraphWidget<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let scene = Scene::new()
            .max_inner_size([10000.0, 10000.0])
            .zoom_range(egui::Rangef::new(0.0, 1.0));

        scene
            .show(ui, self.scene_rect, |ui| {
                for relation in self.model.relation_compositions() {
                    let response = ui.add(RelationWidget {
                        path: self
                            .visualization_controller
                            .get_edge_path(relation.id())
                            .unwrap(),
                        relation: &relation,
                        is_selected: match self.selected {
                            Selection::Relation(id) => *id == relation.id(),
                            _ => false,
                        },
                    });
                    if response.clicked() {
                        *self.selected = Selection::Relation(relation.id());
                    }
                    if response.hovered() {
                        println!("hover: {:?}", relation.id())
                    }
                }
                for class in self.model.classes() {
                    let response = ui.put(
                        egui::Rect::from_center_size(
                            self.visualization_controller
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

                    #[cfg(debug_assertions)]
                    {
                        if self.app_state.debug.show_node_labels {
                            let rect = Rect::from_center_size(
                                response.rect.center_bottom(),
                                egui::vec2(200.0, 100.0),
                            );

                            let debug_text = egui::RichText::new(format!(
                                "ID: {}\nLAYER: {}\nSORT IDX: {}",
                                class.id.to_string(),
                                self.visualization_controller
                                    .get_node_layer(&class.id)
                                    .map_or(String::from("-"), |l| l.to_string()),
                                self.visualization_controller
                                    .get_node_sort_idx(&class.id)
                                    .map_or(String::from("-"), |l| l.to_string())
                            ))
                            .italics()
                            .color(egui::Color32::RED);
                            ui.put(rect, egui::Label::new(debug_text).selectable(false));
                        };
                    }

                    if response.clicked() {
                        *self.selected = Selection::ComplexityClass(class.id);
                    }
                }
            })
            .response
    }
}
