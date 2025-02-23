use egui::{Color32, Pos2, Stroke, Widget};

pub struct RelationWidget {
    pub from: Pos2,
    pub to: Pos2,
}

impl Widget for RelationWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.painter()
            .line_segment([self.from, self.to], Stroke {color: Color32::from_gray(100), width: 2.0});
        ui.response()
    }
}
