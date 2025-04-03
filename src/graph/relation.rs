use egui::{emath::Rot2, epaint::TextShape, FontSelection, Pos2, Rect, Widget};

use crate::model::relation::Relation;

pub struct RelationWidget<'a> {
    pub from: Pos2,
    pub to: Pos2,
    pub relation: &'a Relation,
}

impl Widget for RelationWidget<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let visuals = ui.style().noninteractive();

        ui.painter()
            .line_segment([self.from, self.to], visuals.fg_stroke);
        let relation_label = match self.relation {
            Relation::Subset(_) => "⊆",
            Relation::Equal(_, _) => "=",
            Relation::Unknown => "Ukn",
        };

        let galley = {
            let font_id = FontSelection::Default.resolve(ui.style());
            ui.painter()
                .layout_no_wrap(relation_label.to_string(), font_id, visuals.text_color())
        };

        let text_angle = (self.to - self.from).angle();

        let bounding_rect = Rect::from_center_size(Pos2::ZERO, galley.size())
            .rotate_bb(Rot2::from_angle(text_angle))
            .translate(0.5 * self.from.to_vec2() + 0.5 * self.to.to_vec2());

        if ui.is_rect_visible(bounding_rect) {
            ui.painter().circle(
                bounding_rect.center(),
                0.5 * f32::max(bounding_rect.width(), bounding_rect.height()),
                visuals.bg_fill,
                visuals.bg_stroke,
            );
            let label_pos =
                bounding_rect.center() - (Rot2::from_angle(text_angle) * (galley.size() / 2.0));
            ui.painter().add(
                TextShape::new(label_pos, galley, visuals.text_color()).with_angle(text_angle),
            );
        }

        ui.response()
    }
}
