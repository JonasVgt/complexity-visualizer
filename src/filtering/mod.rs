use egui::Align2;

use crate::{database::complexity_class::Tag, model::filter::Filter};
pub struct FilterState {
    is_open: bool,
    popup_pos: egui::Pos2,
}

impl FilterState {
    pub fn new() -> Self {
        Self {
            is_open: false,
            popup_pos: egui::Pos2::ZERO,
        }
    }

    pub fn ui_filter_window_button(&mut self, ui: &mut egui::Ui) {
        let button = ui.button("󰈲");
        if button.clicked() {
            self.is_open = !self.is_open;
        }

        self.popup_pos = button.rect.right_bottom();
    }

    pub fn ui_filter_popup(&mut self, ctx: &egui::Context, filter: &mut Filter) {
        if self.is_open {
            egui::Area::new(egui::Id::new("popup_area"))
                .pivot(Align2::RIGHT_TOP)
                .fixed_pos(self.popup_pos) // Set exact position
                .show(ctx, |ui| {
                    egui::Frame::NONE
                        .fill(ui.visuals().panel_fill)
                        .show(ui, |ui| {
                            ui.heading("Filter");
                            ui.separator();
                            ui.label("Tags:");
                            for tag in Tag::tags() {
                                if matches!(tag, Tag::Complement) {
                                    continue;
                                }
                                if ui
                                    .checkbox(filter.tag_get_mut(&tag), tag.to_string())
                                    .changed()
                                {
                                    filter.redraw();
                                }
                            }
                            ui.separator();
                            if ui
                                .checkbox(&mut filter.show_complements, "Complements")
                                .changed()
                            {
                                filter.redraw();
                            }
                        });
                });
        }
    }
}
