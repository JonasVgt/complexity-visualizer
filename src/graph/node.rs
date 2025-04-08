use egui::{epaint::TextShape, vec2, Align2, FontSelection, Widget};

use crate::{database::complexity_class::Tag, utils::text_parser::RichTextParser};

pub struct NodeWidget {
    pub label: String,
    pub is_selected: bool,
    pub tags: Vec<Tag>,
}

fn tags_str(tags: &[Tag]) -> String {
    tags.iter()
        .map(|tag| match tag {
            Tag::Probabilistic => "",
            Tag::Nondeterministic => "",
            Tag::Deterministic => "󰁔",
            Tag::Space => "",
            Tag::Time => "",
        })
        .collect::<Vec<&str>>()
        .join(" ")
}

impl Widget for NodeWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        // 1. Deciding widget size:
        let desired_size = ui.spacing().interact_size.y * egui::vec2(5.0, 5.0);
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        // Attach some meta-data to the response which can be used by screen readers:
        response
            .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, &self.label));

        // Paint:
        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact_selectable(&response, self.is_selected);
            let radius = 0.5 * rect.height();
            ui.painter().circle(
                rect.center(),
                0.75 * radius,
                visuals.bg_fill,
                visuals.fg_stroke,
            );
            let label_layout = RichTextParser::new().parse(self.label).into_layout();
            let galley = ui.painter().layout_job(label_layout);
            let galley_size = galley.size();
            ui.painter().add(TextShape::new(
                rect.center() - galley.size() * 0.5,
                galley,
                visuals.text_color(),
            ));

            // Paint Tags
            let font_id = FontSelection::Default.resolve(ui.style());
            let text = tags_str(&self.tags);
            ui.painter().text(
                rect.center() + vec2(0.0, galley_size.y * 0.5),
                Align2::CENTER_TOP,
                text,
                font_id,
                visuals.text_color(),
            );
        }
        response
    }
}
