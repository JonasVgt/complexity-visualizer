use node::node_ui;

use crate::database::MyDatabase;

mod node;

pub fn graph_ui(ui: &mut egui::Ui) -> egui::Response {
    let database = MyDatabase::new();

    let classes = database.fetch_complexity_classes().unwrap();

    classes
        .into_iter()
        .map(|class| node_ui(ui, &class.name))
        .reduce(|acc, e| acc.union(e))
        .unwrap()
}
