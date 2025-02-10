use egui::Align2;

pub fn node_ui(ui: &mut egui::Ui, label: &str) -> egui::Response {
    // 1. Deciding widget size:
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 2.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    // Attach some meta-data to the response which can be used by screen readers:
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, label));

    // Paint:
    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact(&response);
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
            label,
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
