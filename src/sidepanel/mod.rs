use crate::{
    database::complexity_class::ComplexityClass, rich_label, utils::text_parser::RichTextParser,
};

pub fn ui_sidepanel(ui: &mut egui::Ui, class: Option<&ComplexityClass>) {
    if let Some(c) = class {
        ui.horizontal(|ui| {
            for name in &c.names {
                let label = RichTextParser::new()
                    .parse(name.clone())
                    .text_syle(egui::TextStyle::Heading)
                    .to_label();
                ui.add(label);
                ui.add_space(10.0);
            }
        });
        ui.separator();
        ui.add(rich_label!(c.description.clone()));
        ui.separator();
        ui.hyperlink(&c.wikipedia);
    }
}
