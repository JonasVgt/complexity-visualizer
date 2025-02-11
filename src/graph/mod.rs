use egui::ScrollArea;
use node::NodeWidget;

use crate::database::{complexity_class::ComplextiyClass, MyDatabase};

mod node;

pub fn graph_ui(ui: &mut egui::Ui, selected_class: &mut ComplextiyClass) {
    let database = MyDatabase::new();

    let classes = database.fetch_complexity_classes().unwrap();

    ScrollArea::both().show(ui, |ui| {
        let available_size = ui.available_size();
        ui.set_min_size(available_size);

        classes
            .into_iter()
            .map(|class| {
                let response = ui.add(NodeWidget {
                    label: class.name.clone(),
                });
                if response.clicked() {
                    *selected_class = class;
                }
                response
            })
            .reduce(|acc, e| acc.union(e))
            .unwrap();
    });
}
