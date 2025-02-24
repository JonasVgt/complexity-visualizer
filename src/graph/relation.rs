use egui::{Align2, Color32, FontId, Pos2, Stroke, Widget};

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
        ui.painter().text(
            Pos2::from(0.5 * self.from + 0.5 * self.to.to_vec2()),
            Align2::CENTER_CENTER,
            relation_label,
            FontId::default(),
            Color32::WHITE,
        );
        ui.response()
    }
}
