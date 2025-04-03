use crate::{
    database::complexity_class::Tag, model::complexity_class::ComplexityClass, rich_label,
    rich_label_heading, utils::text_parser::RichTextParser,
};

pub fn ui_sidepanel(ui: &mut egui::Ui, class: Option<&ComplexityClass>) {
    if let Some(c) = class {
        ui.add(rich_label_heading!(c.names.first().unwrap().clone()));
        ui.horizontal(|ui| {
            for name in &c.names[1..] {
                ui.add(rich_label!(name.clone()));
                ui.add_space(10.0);
            }
        });
        ui.separator();
        ui.add(rich_label!(c.description.clone()));
        ui.separator();
        ui.hyperlink(&c.wikipedia);
        ui.separator();
        let text = c
            .tags
            .iter()
            .map(|tag| match tag {
                Tag::Deterministic => "Deterministic",
                Tag::Nondeterministic => "Non-deterministic",
                Tag::Probabilistic => "Probabilistic",
                Tag::Space => "Space",
                Tag::Time => "Time",
            })
            .collect::<Vec<&str>>()
            .join(", ");
        ui.label(text);
    }
}
