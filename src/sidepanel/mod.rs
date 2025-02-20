use crate::database::complexity_class::ComplexityClass;

pub fn ui_sidepanel(ui: &mut egui::Ui, class: Option<&ComplexityClass>) {
    if let Some(c) = class {
        let heading = c.names.join(" | ");
        ui.heading(&heading);
        ui.separator();
        ui.label(&c.description);
        ui.separator();
        ui.hyperlink(&c.wikipedia);
    }
}
