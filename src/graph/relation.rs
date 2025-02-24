use egui::{
    emath::Rot2, epaint::TextShape, Color32, FontSelection, Pos2, Rect,
    Stroke, Widget,
};

use crate::database::relation::RelationType;

pub struct RelationWidget {
    pub from: Pos2,
    pub to: Pos2,
    pub relation_type: RelationType,
}

impl Widget for RelationWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.painter().line_segment(
            [self.from, self.to],
            Stroke {
                color: Color32::from_gray(100),
                width: 2.0,
            },
        );
        let relation_label = match self.relation_type {
            RelationType::Subset => "⊆",
            _ => "",
        };

        let galley = {
            let color = ui.style().visuals.text_color();
            let font_id = FontSelection::Default.resolve(ui.style());
            ui.painter()
                .layout_no_wrap(relation_label.to_string(), font_id, color)
        };

        let text_angle =  (self.to - self.from).angle();

        let bounding_rect = Rect::from_center_size(Pos2::ZERO, galley.size())
            .rotate_bb(Rot2::from_angle(text_angle))
            .translate(0.5 * self.from.to_vec2() + 0.5 * self.to.to_vec2());

        if ui.is_rect_visible(bounding_rect) {
            let pos = bounding_rect.center() - (Rot2::from_angle(text_angle) * (galley.size() / 2.0));
            ui.painter()
                .add(TextShape::new(pos, galley, Color32::default()).with_angle(text_angle));
        }

        ui.response()
    }
}
