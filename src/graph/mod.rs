use node::node_ui;

mod node;

pub fn graph_ui(ui: &mut egui::Ui) -> egui::Response {
    let response = node_ui(ui, "test");
    response.union(node_ui(ui, "abc"));
    
    
    response
}