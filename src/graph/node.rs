use egui::{Align2, Widget};

pub struct NodeWidget {
    pub label: String,
    pub selected: bool,
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
            let visuals = ui.style().interact_selectable(&response, self.selected);
            let radius = 0.5 * rect.height();
            ui.painter().circle(
                rect.center(),
                0.75 * radius,
                visuals.bg_fill,
                visuals.fg_stroke,
            );
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                self.label,
                ui.style()
                    .text_styles
                    .get(&egui::TextStyle::Button)
                    .cloned()
                    .unwrap(),
                visuals.text_color(),
            );
        }
        response
    }
}
