use egui::{Pos2, Widget};

pub struct RelationWidget {
    pub from: Pos2,
    pub to: Pos2,
}

impl Widget for RelationWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.painter()
            .arrow(self.from, self.to - self.from, ui.visuals().window_stroke());
        ui.response()
    }
}
