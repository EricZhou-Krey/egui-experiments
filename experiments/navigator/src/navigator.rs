use crate::{
    graph::{
        triangulation::graph::TriangulationGraph, Graph, GraphInteractNodeIndex, GraphMode,
        GraphUpdate,
    },
    layouts::{ExperimentIndex, Layout},
    settings::{
        style_sheet::{set_font, BYTES_0XPROTONERDFONT},
        InteractableTriangulationMeshSettings, NavigatorSettings, TriangulationGraphSettings,
    },
    terminal::NavigatorTerminal,
};
use egui::{Rect, Visuals};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum NavigatorUpdate {
    Open(ExperimentIndex),
}

pub struct Navigator {
    pub terminal: NavigatorTerminal,
    pub settings: NavigatorSettings,
    pub graph_mode: GraphMode,
    pub graph: Graph,
    pub experiment_overlay: Option<Layout>,
    displayed_overlay: Option<Layout>,
    overlay_transition_t: f32,
    pub update_status: Option<NavigatorUpdate>,
    show_settings: bool,
    settings_popup_t: f32,
}

impl Default for Navigator {
    fn default() -> Self {
        Self::new()
    }
}

impl Navigator {
    pub fn new() -> Self {
        let mut graph: Graph = Graph::Triangulation(Box::new(TriangulationGraph::new(
            TriangulationGraphSettings::default(),
            InteractableTriangulationMeshSettings {
                n_interactable: Layout::ALL.len(),
                ..Default::default()
            },
        )));
        graph.set_interact_index(Some(GraphInteractNodeIndex::default()));

        Self {
            terminal: NavigatorTerminal::default(),
            settings: NavigatorSettings::default(),
            graph_mode: GraphMode::Triangulation,
            graph,
            experiment_overlay: Some(Layout::default()),
            displayed_overlay: Some(Layout::default()),
            overlay_transition_t: 1.0,
            update_status: None,
            show_settings: false,
            settings_popup_t: 0.0,
        }
    }
}

impl Navigator {
    fn setup_visuals(&self, ui: &mut egui::Ui) {
        set_font(
            ui.ctx(),
            "0xProtoNerdFont".to_string(),
            BYTES_0XPROTONERDFONT,
        );

        let visuals: &mut Visuals = &mut ui.style_mut().visuals;

        visuals.selection.bg_fill = self.settings.active_tab_bg;
        visuals.widgets.inactive.weak_bg_fill = self.settings.inactive_tab_bg;
        visuals.widgets.hovered.weak_bg_fill = self.settings.inactive_tab_bg;
    }

    fn settings_ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("SETTINGS");
            ui.separator();

            egui::CollapsingHeader::new("Navigator Settings")
                .default_open(true)
                .show(ui, |ui| {
                    ui.label("Animation & Layout");
                    ui.add(
                        egui::Slider::new(&mut self.settings.settings_popup_size, 0.1..=1.0)
                            .text("Popup Size"),
                    );
                    ui.add(
                        egui::Slider::new(
                            &mut self.settings.settings_popup_animation_time,
                            0.0..=2.0,
                        )
                        .text("Popup Anim Time"),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.settings.layout_animation_time, 0.0..=2.0)
                            .text("Layout Anim Time"),
                    );

                    ui.separator();
                    ui.label("Tab Colors");
                    ui.horizontal(|ui| {
                        ui.color_edit_button_srgba(&mut self.settings.active_tab_bg);
                        ui.label("Active Tab BG");
                    });
                    ui.horizontal(|ui| {
                        ui.color_edit_button_srgba(&mut self.settings.active_tab_text);
                        ui.label("Active Tab Text");
                    });
                    ui.horizontal(|ui| {
                        ui.color_edit_button_srgba(&mut self.settings.inactive_tab_bg);
                        ui.label("Inactive Tab BG");
                    });
                    ui.horizontal(|ui| {
                        ui.color_edit_button_srgba(&mut self.settings.inactive_tab_text);
                        ui.label("Inactive Tab Text");
                    });

                    ui.separator();
                    ui.label("Frame Backgrounds");
                    ui.horizontal(|ui| {
                        ui.color_edit_button_srgba(&mut self.settings.top_panel_frame.fill);
                        ui.label("Top Panel Frame");
                    });
                    ui.horizontal(|ui| {
                        ui.color_edit_button_srgba(&mut self.settings.graph_outer_frame.fill);
                        ui.label("Graph Outer Frame");
                    });
                    ui.horizontal(|ui| {
                        ui.color_edit_button_srgba(&mut self.settings.graph_inner_frame.fill);
                        ui.label("Graph Inner Frame");
                    });
                    ui.horizontal(|ui| {
                        ui.color_edit_button_srgba(&mut self.settings.terminal_frame.fill);
                        ui.label("Terminal Frame");
                    });
                    ui.horizontal(|ui| {
                        ui.color_edit_button_srgba(&mut self.settings.settings_popup_frame.fill);
                        ui.label("Settings Popup Frame");
                    });
                });

            ui.separator();

            self.graph.settings_ui(ui);

            if ui.button("Reset").clicked() {
                self.settings = NavigatorSettings::default();
                self.graph.reset_settings();
            }
        });
    }

    fn settings_popup(&mut self, ui: &mut egui::Ui) {
        let delta: f32 = ui.input(|i| i.stable_dt);
        let animation_duration: f32 = self.settings.settings_popup_animation_time;
        self.settings_popup_t +=
            (delta / animation_duration) * if self.show_settings { 1.0 } else { -1.0 };

        self.settings_popup_t = self.settings_popup_t.clamp(0.0, 1.0);

        if self.settings_popup_t == 0.0 {
            return;
        }

        let eased_t: f32 = egui::emath::easing::quadratic_in_out(self.settings_popup_t);
        let screen_width: f32 = ui.viewport_rect().size().x;

        let target_width: f32 = screen_width * self.settings.settings_popup_size;
        let animated_width: f32 = target_width * eased_t;

        let mut panel =
            egui::Panel::left("settings_popup").frame(self.settings.settings_popup_frame);

        if self.settings_popup_t >= 1.0 {
            panel = panel.resizable(true).size_range(0.0..=target_width);
        } else {
            panel = panel.resizable(false).exact_size(animated_width);
        }

        panel.show(ui, |ui: &mut egui::Ui| {
            self.settings_ui(ui);
        });

        if self.settings_popup_t > 0.0 && self.settings_popup_t < 1.0 {
            ui.ctx().request_repaint();
        }
    }

    fn settings_bar(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("settings_panel")
            .frame(self.settings.top_panel_frame)
            .show(ui, |ui: &mut egui::Ui| {
                ui.horizontal(|ui: &mut egui::Ui| {
                    let mut new_mode: GraphMode = self.graph_mode.clone();

                    let tab_text = |text: &str, is_active: bool| {
                        let text_color = if is_active {
                            self.settings.active_tab_text
                        } else {
                            self.settings.inactive_tab_text
                        };
                        egui::RichText::new(text).color(text_color).strong()
                    };

                    let setttings_text: egui::RichText = tab_text("Settings", self.show_settings);
                    ui.toggle_value(&mut self.show_settings, setttings_text);

                    ui.separator();

                    ui.selectable_value(
                        &mut new_mode,
                        GraphMode::Triangulation,
                        tab_text("Triangulation", self.graph_mode == GraphMode::Triangulation),
                    );
                    ui.selectable_value(
                        &mut new_mode,
                        GraphMode::Boids,
                        tab_text("Boids", self.graph_mode == GraphMode::Boids),
                    );
                    ui.selectable_value(
                        &mut new_mode,
                        GraphMode::Life,
                        tab_text("Game of Life", self.graph_mode == GraphMode::Life),
                    );

                    if new_mode != self.graph_mode {
                        self.graph_mode = new_mode.clone();
                        self.graph = match new_mode {
                            GraphMode::Triangulation => Graph::Triangulation(Box::default()),
                            GraphMode::Boids => Graph::Boids(Box::default()),
                            GraphMode::Life => Graph::Life(Box::default()),
                        };
                    }
                });
            });
    }

    fn graph_ui(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default()
            .frame(self.settings.graph_outer_frame)
            .show(ui, |ui: &mut egui::Ui| {
                let panel_rect: Rect = ui.max_rect();

                self.settings.graph_inner_frame.show(ui, |ui| {
                    if let Some(graph_update) = self.graph.ui(ui) {
                        match graph_update {
                            GraphUpdate::Deselect => {
                                self.experiment_overlay = None;
                            }
                            GraphUpdate::Select(index) => {
                                self.experiment_overlay = Some(Layout::ALL[index.0].clone());
                            }
                            GraphUpdate::Reselect(index) => {
                                if let Some(experiment_index) =
                                    ExperimentIndex::L_INDEX_TO_EXPERIMENT_INDEX[index.0]
                                {
                                    self.update_status =
                                        Some(NavigatorUpdate::Open(experiment_index));
                                }
                            }
                        }
                    }
                });

                if self.experiment_overlay != self.displayed_overlay {
                    let delta: f32 = ui.input(|i| i.stable_dt);
                    let animation_duration: f32 = self.settings.layout_animation_time;
                    self.overlay_transition_t += delta / animation_duration;

                    if self.overlay_transition_t >= 1.0 {
                        self.overlay_transition_t = 1.0;
                        self.displayed_overlay = self.experiment_overlay.clone();
                    } else {
                        ui.ctx().request_repaint();
                    }
                } else {
                    self.overlay_transition_t = 0.0;
                }

                let mut overlay_ui: egui::Ui =
                    ui.new_child(egui::UiBuilder::new().max_rect(panel_rect));

                let mut blocked_screen_rects: Vec<Rect> = Vec::new();
                if let Some(overlay) = &self.experiment_overlay {
                    blocked_screen_rects.extend(
                        overlay
                            .panels(panel_rect)
                            .into_iter()
                            .map(|(r, _)| r)
                            .collect::<Vec<Rect>>(),
                    );
                }

                self.graph
                    .clip_interactable_nodes(blocked_screen_rects, panel_rect);

                Layout::ui(
                    &mut overlay_ui,
                    &self.displayed_overlay,
                    &self.experiment_overlay,
                    self.overlay_transition_t,
                );
            });
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<NavigatorUpdate> {
        self.setup_visuals(ui);
        self.update_status = None;

        self.settings_bar(ui);

        egui::Panel::bottom("terminal_panel")
            .frame(self.settings.terminal_frame)
            .resizable(true)
            .show(ui, |ui: &mut egui::Ui| {
                if let Some((command_fn, args)) = self.terminal.ui(ui) {
                    command_fn(self, &args);
                }
            });
        egui::CentralPanel::no_frame().show(ui, |ui: &mut egui::Ui| {
            self.settings_popup(ui);
            self.graph_ui(ui);
        });

        self.update_status
    }

    pub fn logic(&mut self, ctx: &egui::Context) {
        self.graph.logic(ctx);
    }

    pub fn raw_input_hook(&mut self, _ctx: &egui::Context, _raw_input: &mut egui::RawInput) {}
}

impl eframe::App for Navigator {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.ui(ui);
    }

    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.logic(ctx);
    }

    fn raw_input_hook(&mut self, ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        self.raw_input_hook(ctx, raw_input);
    }
}
