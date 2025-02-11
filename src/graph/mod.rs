use node::node_ui;

use crate::database::{complexity_class::ComplextiyClass, MyDatabase};

mod node;

pub fn graph_ui(ui: &mut egui::Ui, selected_class: &mut ComplextiyClass) -> egui::Response {
    let database = MyDatabase::new();

    let classes = database.fetch_complexity_classes().unwrap();

    classes
        .into_iter()
        .map(|class| {
            let response = node_ui(ui, &class.name);
            if response.clicked() {
                *selected_class = class;
            }
            response
        })
        .reduce(|acc, e| acc.union(e))
        .unwrap()
}
