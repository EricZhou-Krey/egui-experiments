use crate::{
    boids::graph::BoidGraph,
    layouts::Layout,
    life::graph::LifeGraph,
    settings::{
        logic_sheet::LAYOUT_ANIMATION_TIME,
        style_sheet::{set_font, BYTES_0XPROTONERDFONT, MIN_TERMINAL_SIZE},
        InteractableTriangulationMeshSettings, NavigatorSettings, TriangulationGraphSettings,
    },
    terminal::NavigatorTerminal,
    triangulation::graph::TriangulationGraph,
};
use eframe::{egui, App};
use egui::{Pos2, Rect};
use glam::Vec2;

#[derive(Debug, Clone, PartialEq)]
pub enum Tab {
    Settings,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum GraphMode {
    #[default]
    Triangulation,
    Boids,
    Life,
}

pub enum Graph {
    Triangulation(Box<TriangulationGraph>),
    Boids(Box<BoidGraph>),
    Life(Box<LifeGraph>),
}

impl Graph {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        match self {
            Self::Triangulation(bg) => bg.ui(ui, frame),
            Self::Life(bg) => bg.ui(ui, frame),
            Self::Boids(bg) => bg.ui(ui, frame),
        }
    }

    fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self {
            Self::Triangulation(bg) => bg.logic(ctx, frame),
            Self::Life(bg) => bg.logic(ctx, frame),
            Self::Boids(bg) => bg.logic(ctx, frame),
        }
    }

    pub fn interact_index(&mut self) -> Option<usize> {
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

    pub fn set_interact_index(&mut self, index: Option<usize>) {
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
}

pub struct Navigator {
    pub terminal: NavigatorTerminal,
    pub settings: NavigatorSettings,
    pub graph_mode: GraphMode,
    pub graph: Graph,
    pub experiment_overlay: Option<Layout>,
    displayed_overlay: Option<Layout>,
    overlay_transition_t: f32,
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
        graph.set_interact_index(Some(0));

        Self {
            terminal: NavigatorTerminal::default(),
            settings: NavigatorSettings::default(),
            graph_mode: GraphMode::Triangulation,
            graph,
            experiment_overlay: Some(Layout::default()),
            displayed_overlay: Some(Layout::default()),
            overlay_transition_t: 1.0,
        }
    }
}

impl eframe::App for Navigator {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        set_font(
            ui.ctx(),
            "0xProtoNerdFont".to_string(),
            BYTES_0XPROTONERDFONT,
        );
        egui::Panel::top("settings_panel")
            .frame(self.settings.top_panel_frame)
            .show(ui, |ui: &mut egui::Ui| {
                ui.horizontal(|ui: &mut egui::Ui| {
                    let visuals = &mut ui.style_mut().visuals;

                    visuals.selection.bg_fill = self.settings.active_tab_bg;
                    visuals.widgets.inactive.weak_bg_fill = self.settings.inactive_tab_bg;
                    visuals.widgets.hovered.weak_bg_fill = self.settings.inactive_tab_bg;

                    let mut new_mode: GraphMode = self.graph_mode.clone();

                    let tab_text = |text: &str, is_active: bool| {
                        let text_color = if is_active {
                            self.settings.active_tab_text
                        } else {
                            self.settings.inactive_tab_text
                        };
                        egui::RichText::new(text).color(text_color).strong()
                    };

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

        egui::Panel::bottom("terminal_panel")
            .frame(self.settings.terminal_frame)
            .min_size(MIN_TERMINAL_SIZE)
            .resizable(true)
            .show(ui, |ui: &mut egui::Ui| {
                if let Some((command_fn, args)) = self.terminal.ui(ui) {
                    command_fn(self, &args);
                }
            });

        egui::CentralPanel::default()
            .frame(self.settings.graph_outer_frame)
            .show(ui, |ui: &mut egui::Ui| {
                let panel_rect: Rect = ui.max_rect();

                self.settings
                    .graph_inner_frame
                    .show(ui, |ui| self.graph.ui(ui, frame));

                if self.experiment_overlay != self.displayed_overlay {
                    let delta: f32 = ui.input(|i| i.stable_dt);
                    let animation_duration: f32 = LAYOUT_ANIMATION_TIME;
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

                if self
                    .experiment_overlay
                    .as_ref()
                    .map(|layout| layout.clone() as usize)
                    != self.graph.interact_index()
                {
                    self.experiment_overlay =
                        self.graph.interact_index().map(|i| Layout::ALL[i].clone());
                }

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

    fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.graph.logic(ctx, frame);
    }
}

// TODO: LOAD readmes and display title, preview window and etc, on hover, popup, set velocity to 0
// on selection and reset otherwise, and etc

/*

TODO: after first experiment is completed add to this navigator the project ui and overlay, then
make the navigator assign ids to the overlays and let the app choose which overlays correspond
to each of the tab enums and then make it interactable as to which one is clicked on and etc

*/
