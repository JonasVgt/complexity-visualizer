
#[allow(dead_code)]
pub fn ui_debugging_panel(ui: &mut egui::Ui, enabled: &mut bool) {
    ui.checkbox(enabled, "Debug");
}