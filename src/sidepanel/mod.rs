use crate::{
    database::complexity_class::ComplexityClass, rich_label, utils::text_parser::RichTextParser,
};

pub fn ui_sidepanel(ui: &mut egui::Ui, class: Option<&ComplexityClass>) {
    if let Some(c) = class {
        let heading = c.names.join(" | ");
        ui.heading(&heading);
        ui.separator();
        ui.add(rich_label!(c.description.clone()));
        ui.separator();
        ui.hyperlink(&c.wikipedia);
    }
}
