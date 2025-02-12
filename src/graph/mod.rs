use egui::{pos2, Rect, Scene};
use node::NodeWidget;

use crate::database::{complexity_class::ComplextiyClass, MyDatabase};

mod node;

pub fn graph_ui(ui: &mut egui::Ui, selected_class: &mut ComplextiyClass) {
    let database = MyDatabase::new();

    let classes = database.fetch_complexity_classes().unwrap();

    Scene::new().show(
        ui,
        &mut Rect::from_min_size(
            pos2(0.0, 0.0),
            egui::Vec2 {
                x: 1000.0,
                y: 1000.0,
            },
        ),
        |ui| {
            let mut i = 0;
            for class in classes {
                let pos = egui::pos2((200 * i) as f32, (100 * i) as f32);
                let response = ui.put(
                    egui::Rect::from_center_size(pos, egui::vec2(50.0, 50.0)),
                    NodeWidget {
                        label: class.name.clone(),
                    },
                );
                if response.clicked() {
                    *selected_class = class;
                }
                i += 1;
            }
        },
    );
}
