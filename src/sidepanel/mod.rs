use crate::database::complexity_class::ComplexityClass;

pub fn ui_sidepanel(ui: &mut egui::Ui, class: Option<&ComplexityClass>) {
    if let Some(c) = class {
        ui.heading(&c.name);
        ui.separator();
        ui.label(&c.description);
        ui.separator();
        ui.hyperlink(&c.wikipedia);
    }
}
