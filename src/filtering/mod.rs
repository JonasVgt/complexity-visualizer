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

                            // Computational Model (Deterministic TM, Nondeterministic TM, ...)
                            ui.separator();
                            ui.label("Computational Model");
                            if ui
                                .checkbox(filter.tag_get_mut(&Tag::Deterministic), "Deterministic TM")
                                .on_hover_text(
                                    "Show complexity classes that are defined over deterministic Turing machines (e.g. P, EXPSPACE, etc.)",
                                )
                                .changed()
                            {
                                filter.redraw();
                            }
                            if ui
                                .checkbox(filter.tag_get_mut(&Tag::Nondeterministic), "Nondeterministic TM")
                                .on_hover_text(
                                    "Show complexity classes that are defined over deterministic Turing machines (e.g. NP, NEXPSPACE, etc.)",
                                )
                                .changed()
                            {
                                filter.redraw();
                            }
                            if ui
                                .checkbox(filter.tag_get_mut(&Tag::Probabilistic), "Probabilistic TM")
                                .on_hover_text(
                                    "Show complexity classes that are defined over deterministic Turing machines (e.g. PP, BPP, etc.)",
                                )
                                .changed()
                            {
                                filter.redraw();
                            }

                            // Resource (Time, Space, ...)
                            ui.separator();
                            ui.label("Resource");
                            if ui
                                .checkbox(filter.tag_get_mut(&Tag::Time), "Time")
                                .on_hover_text(
                                    "Show complexity classes that bound the usage of runtime (e.g. P, EXPTIME, etc.)",
                                )
                                .changed()
                            {
                                filter.redraw();
                            }
                            if ui
                                .checkbox(filter.tag_get_mut(&Tag::Space), "Space")
                                .on_hover_text(
                                    "Show complexity classes that bound the usage of memory space (e.g. L, EXPSPACE, etc.)",
                                )
                                .changed()
                            {
                                filter.redraw();
                            }

                            // Complements
                            ui.separator();
                            ui.label("Additional CLasses");
                            if ui
                                .checkbox(filter.tag_get_mut(&Tag::Complement), "Complements")
                                .on_hover_text(
                                    "Show complements of complexity classes (i.e. co-X classes)",
                                )
                                .changed()
                            {
                                filter.redraw();
                            }
                        });
                });
        }
    }
}
