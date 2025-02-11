use crate::database::complexity_class::ComplextiyClass;

pub fn ui_sidepanel(ui : &mut egui::Ui, class: ComplextiyClass){
    ui.heading(class.name);
    ui.separator();
    ui.label(class.description);
}