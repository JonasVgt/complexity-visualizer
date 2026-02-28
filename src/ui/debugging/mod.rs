use crate::ui::AppState;

pub struct DebugAppState {
    pub show_node_labels: bool,
    pub show_dummy_nodes: bool,
}

impl DebugAppState {
    pub fn new() -> Self {
        DebugAppState {
            show_node_labels: false,
            show_dummy_nodes: false,
        }
    }
}

pub fn ui_debugging_panel(ui: &mut egui::Ui, appstate: &mut AppState) {
    ui.checkbox(&mut appstate.debug.show_node_labels, "Show node labels");
    ui.checkbox(&mut appstate.debug.show_dummy_nodes, "Show dummy nodes");
}
