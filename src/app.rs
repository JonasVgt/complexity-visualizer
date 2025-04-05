use egui::{pos2, FontData, FontDefinitions, FontFamily, Rect};

use crate::{filtering::FilterState, graph::GraphWidget, model::Model, sidepanel::ui_sidepanel};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ComplexityVisualizerApp {
    selected_class: Option<u64>,

    #[serde(skip)]
    model: Model,

    scene_rect: Rect,

    #[serde(skip)]
    filter_state: FilterState,
}

impl Default for ComplexityVisualizerApp {
    fn default() -> Self {
        Self {
            selected_class: None,
            model: Model::new(),
            scene_rect: Rect::from_min_size(
                pos2(0.0, 0.0),
                egui::Vec2 {
                    x: 1000.0,
                    y: 1000.0,
                },
            ),
            filter_state: FilterState::new(),
        }
    }
}

impl ComplexityVisualizerApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for ComplexityVisualizerApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set fonts
        let mut fonts = FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters):
        fonts.font_data.insert(
            "JetBrainsMono".to_owned(),
            std::sync::Arc::new(
                // .ttf and .otf supported
                FontData::from_static(include_bytes!(
                    "../assets/fonts/JetBrainsMonoNerdFont-Medium.ttf"
                )),
            ),
        );

        // Put my font first (highest priority):
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "JetBrainsMono".to_owned());

        // Put my font as last fallback for monospace:
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("JetBrainsMono".to_owned());

        ctx.set_fonts(fonts);

        self.model.update();

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        if self.selected_class.is_some() {
            let sidepanel_width = f32::min(ctx.available_rect().width() * 0.33, 300.0);
            egui::SidePanel::right("my_right_panel")
                .default_width(sidepanel_width)
                .show(ctx, |ui| {
                    let class = self.selected_class.and_then(|id| self.model.get_class(id));
                    ui_sidepanel(ui, class)
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.horizontal(|ui| {
                ui.heading("Complexity Classes");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::LEFT), |ui| {
                    self.filter_state.ui_filter_window_button(ui);
                });
            });

            ui.add(GraphWidget {
                selected_class: &mut self.selected_class,
                model: &self.model,
                scene_rect: &mut self.scene_rect,
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                footer(ui);
                egui::warn_if_debug_build(ui);
            });
        });

        self.filter_state
            .ui_filter_popup(ctx, self.model.filter_mut());
    }
}

fn footer(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
        ui.add_space(10.0);
        ui.hyperlink_to(
            "Source code.",
            "https://github.com/JonasVgt/complexity-visualizer",
        );
    });
}
