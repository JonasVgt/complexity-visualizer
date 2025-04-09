use crate::{
    database::complexity_class::Tag,
    model::{
        complexity_class::ComplexityClass,
        relation::{Relation, Subset},
    },
    rich_label, rich_label_heading,
    utils::text_parser::RichTextParser,
};

pub fn ui_sidepanel_relation(ui: &mut egui::Ui, relation: &Relation) {
    let heading = match relation {
        Relation::Equal(Subset { from, to }, _) => {
            format!("{} = {}", from, to)
        }
        Relation::Subset(Subset { from, to }) => {
            format!("{} ⊆ {}", from, to)
        }
    };
    ui.add(rich_label_heading!(heading));
    ui.label("Unfortunately, this feature has not been implemented yet :(");
    ui.separator();
}

pub fn ui_sidepanel_class(ui: &mut egui::Ui, class: &ComplexityClass) {
    ui.add(rich_label_heading!(class.names.first().unwrap().clone()));
    ui.horizontal(|ui| {
        for name in &class.names[1..] {
            ui.add(rich_label!(name.clone()));
            ui.add_space(10.0);
        }
    });
    ui.separator();
    ui.add(rich_label!(class.description.clone()));
    ui.separator();
    ui.hyperlink(&class.wikipedia);
    ui.separator();
    let text = class
        .tags
        .iter()
        .map(|tag| match tag {
            Tag::Deterministic => "Deterministic",
            Tag::Nondeterministic => "Non-deterministic",
            Tag::Probabilistic => "Probabilistic",
            Tag::Space => "Space",
            Tag::Time => "Time",
            Tag::Complement => "Complement",
        })
        .collect::<Vec<&str>>()
        .join(", ");
    ui.label(text);
}
