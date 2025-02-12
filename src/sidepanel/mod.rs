use crate::database::complexity_class::ComplexityClass;

pub fn ui_sidepanel(ui: &mut egui::Ui, class: &ComplexityClass) {
    ui.heading(&class.name);
    ui.separator();
    ui.label(&class.description);
    ui.separator();
    ui.hyperlink(&class.wikipedia_link);
}
