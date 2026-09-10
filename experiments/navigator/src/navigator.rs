use crate::{
    boids::graph::BoidGraph,
    layouts::{ExperimentIndex, Layout},
    life::graph::LifeGraph,
    settings::{
        style_sheet::{set_font, BYTES_0XPROTONERDFONT},
        InteractableTriangulationMeshSettings, NavigatorSettings, TriangulationGraphSettings,
    },
    terminal::NavigatorTerminal,
    triangulation::graph::TriangulationGraph,
};
use egui::{Pos2, Rect, Visuals};
use glam::Vec2;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum GraphMode {
    #[default]
    Triangulation,
    Boids,
    Life,
}

#[derive(Debug)]
pub enum Graph {
    Triangulation(Box<TriangulationGraph>),
    Boids(Box<BoidGraph>),
    Life(Box<LifeGraph>),
}

#[derive(Debug, Default, Clone, PartialEq, Copy)]
pub struct GraphInteractNodeIndex(pub usize);

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum GraphUpdate {
    Deselect,
    Select(GraphInteractNodeIndex),
    Reselect(GraphInteractNodeIndex),
}

impl Graph {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<GraphUpdate> {
        match self {
            Self::Triangulation(bg) => bg.ui(ui),
            Self::Life(bg) => bg.ui(ui),
            Self::Boids(bg) => bg.ui(ui),
        }
    }

    pub fn logic(&mut self, ctx: &egui::Context) {
        match self {
            Self::Triangulation(bg) => bg.logic(ctx),
            Self::Life(bg) => bg.logic(ctx),
            Self::Boids(bg) => bg.logic(ctx),
        }
    }

    pub fn interact_index(&mut self) -> Option<GraphInteractNodeIndex> {
        match self {
            Self::Triangulation(bg) => bg.mesh.interact_vertex,
            Self::Life(_bg) => {
                todo!();
            }
            Self::Boids(_bg) => {
                todo!();
            }
        }
    }

    pub fn set_interact_index(&mut self, index: Option<GraphInteractNodeIndex>) {
        match self {
            Self::Triangulation(bg) => bg.mesh.interact_vertex = index,
            Self::Life(_bg) => {
                todo!();
            }
            Self::Boids(_bg) => {
                todo!();
            }
        }
    }

    pub fn clip_interactable_nodes(
        &mut self,
        blocked_screen_rects: Vec<Rect>,
        allowed_clip_rect: Rect,
    ) {
        match self {
            Self::Triangulation(bg) => {
                let allowed_min: Vec2 = bg.graph_view_transform.to_uv(allowed_clip_rect.min);
                let allowed_max: Vec2 = bg.graph_view_transform.to_uv(allowed_clip_rect.max);
                let allowed_uv_rect: Rect = egui::Rect::from_min_max(
                    egui::pos2(
                        allowed_min.x.min(allowed_max.x),
                        allowed_min.y.min(allowed_max.y),
                    ),
                    egui::pos2(
                        allowed_min.x.max(allowed_max.x),
                        allowed_min.y.max(allowed_max.y),
                    ),
                );

                let blocked_uv_rects: Vec<Rect> = blocked_screen_rects
                    .into_iter()
                    .map(|screen_rect| {
                        let min = bg.graph_view_transform.to_uv(screen_rect.min);
                        let max = bg.graph_view_transform.to_uv(screen_rect.max);
                        egui::Rect::from_min_max(
                            egui::pos2(min.x.min(max.x), min.y.min(max.y)),
                            egui::pos2(min.x.max(max.x), min.y.max(max.y)),
                        )
                    })
                    .collect();

                let interactable_vertices: Vec<usize> = bg.mesh.interactable_vertices.clone();
                let mut current_vertices: Vec<&mut Vec2> =
                    bg.mesh.vertices.iter_mut().map(|v| &mut v.pos).collect();

                for &v_index in &interactable_vertices {
                    let pos: &mut Vec2 = current_vertices[v_index];
                    let pos2: Pos2 = egui::pos2(pos.x, pos.y);

                    let is_blocked: bool = blocked_uv_rects.iter().any(|r| r.contains(pos2));
                    let is_outside: bool = !allowed_uv_rect.contains(pos2);

                    if is_blocked || is_outside {
                        for _ in 0..50 {
                            let rx = allowed_uv_rect.min.x
                                + rand::random::<f32>() * allowed_uv_rect.width();
                            let ry = allowed_uv_rect.min.y
                                + rand::random::<f32>() * allowed_uv_rect.height();
                            let candidate_pos = egui::pos2(rx, ry);

                            if !blocked_uv_rects.iter().any(|r| r.contains(candidate_pos)) {
                                *current_vertices[v_index] = glam::vec2(rx, ry);
                                break;
                            }
                        }
                    }
                }
            }
            Self::Life(_bg) => {
                todo!()
            }
            Self::Boids(_bg) => {
                todo!()
            }
        }
    }

    pub fn settings_ui(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new("Current Graph Settings")
            .default_open(true)
            .show(ui, |ui| match self {
                Graph::Triangulation(bg) => {
                    let mut changed = false;

                    ui.label("Triangulation Settings");
                    changed |= ui
                        .add(
                            egui::Slider::new(&mut bg.settings.mesh_zoom, 0.1..=5.0)
                                .text("Mesh Zoom"),
                        )
                        .changed();

                    ui.separator();
                    ui.label("Mesh Interactable Settings");

                    changed |= ui
                        .add(
                            egui::Slider::new(
                                &mut bg.mesh.settings.n_internal_vertices,
                                Layout::ALL.len()..=500,
                            )
                            .text("N Internal Vertices"),
                        )
                        .changed();

                    changed |= ui
                        .add(
                            egui::Slider::new(&mut bg.mesh.settings.vertex_speed, 0.01..=1.0)
                                .text("Vertex Speed"),
                        )
                        .changed();

                    if changed {
                        bg.apply_settings();
                    }
                }
                Graph::Boids(_bg) => {
                    ui.label("Boids Settings");
                    ui.label("(Add Boids-specific fields here)");
                }
                Graph::Life(_bg) => {
                    ui.label("Game of Life Settings");
                    ui.label("(Add Game of Life-specific fields here)");
                }
            });
    }

    pub fn reset_settings(&mut self) {
        match self {
            Self::Triangulation(bg) => {
                bg.settings = TriangulationGraphSettings::default();
                bg.mesh.settings = InteractableTriangulationMeshSettings {
                    n_interactable: Layout::ALL.len(),
                    ..Default::default()
                };
                bg.apply_settings();
            }
            Self::Life(_bg) => {
                // TODO: Reset Game of Life settings
                // _bg.settings = LifeSettings::default();
                // _bg.apply_settings();
            }
            Self::Boids(_bg) => {
                // TODO: Reset Boids settings
                // _bg.settings = BoidsSettings::default();
                // _bg.apply_settings();
            }
        }
    }
}

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
