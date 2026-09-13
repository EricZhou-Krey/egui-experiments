pub mod boids;
pub mod life;
pub mod triangulation;

use egui::{Pos2, Rect};
use glam::Vec2;

use crate::{
    graph::{
        boids::graph::BoidGraph, life::graph::LifeGraph, triangulation::graph::TriangulationGraph,
    },
    layouts::Layout,
    settings::{
        BoidsGraphSettings, InteractableTriangulationMeshSettings, LifeGraphSettings,
        TriangulationGraphSettings,
    },
};

#[derive(Debug, Default, Clone, PartialEq)]
pub enum GraphMode {
    #[default]
    Triangulation,
    Boids,
    Life,
}

impl GraphMode {
    pub const ALL: [GraphMode; 3] = [GraphMode::Triangulation, GraphMode::Boids, GraphMode::Life];

    pub fn name(&self) -> &'static str {
        match self {
            GraphMode::Triangulation => "triangulation",
            GraphMode::Boids => "boids",
            GraphMode::Life => "life",
        }
    }

    pub fn try_from_name(name: &str) -> Result<Self, &'static str> {
        Self::ALL
            .iter()
            .find(|m| m.name() == name)
            .cloned()
            .ok_or("No graph of name")
    }
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
            Self::Life(bg) => bg.interact_index,
            Self::Boids(bg) => bg.interact_index,
        }
    }

    pub fn set_interact_index(&mut self, index: Option<GraphInteractNodeIndex>) {
        match self {
            Self::Triangulation(bg) => bg.mesh.interact_vertex = index,
            Self::Life(bg) => bg.interact_index = index,
            Self::Boids(bg) => bg.interact_index = index,
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
            Self::Life(bg) => {
                bg.clip_interactable_nodes(blocked_screen_rects, allowed_clip_rect);
            }
            Self::Boids(bg) => {
                bg.clip_interactable_nodes(blocked_screen_rects, allowed_clip_rect);
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
                // Inside `settings_ui()` on your `Graph` enum wrapper
                Graph::Boids(bg) => {
                    let mut changed: bool = false;

                    ui.label("Boids Settings");

                    changed |= ui
                        .add(
                            egui::Slider::new(&mut bg.settings.n_boids, 10..=500)
                                .text("Number of Boids"),
                        )
                        .changed();
                    changed |= ui
                        .add(egui::Slider::new(&mut bg.settings.mesh_zoom, 0.1..=5.0).text("Zoom"))
                        .changed();

                    ui.separator();
                    ui.label("Speed Constraints");

                    changed |= ui
                        .add(
                            egui::Slider::new(&mut bg.settings.min_speed, 0.0..=1.0)
                                .text("Min Speed"),
                        )
                        .changed();
                    changed |= ui
                        .add(
                            egui::Slider::new(&mut bg.settings.max_speed, 0.01..=2.0)
                                .text("Max Speed"),
                        )
                        .changed();

                    // Safety check: Prevent min_speed from exceeding max_speed
                    if bg.settings.max_speed < bg.settings.min_speed {
                        bg.settings.max_speed = bg.settings.min_speed;
                        changed = true;
                    }

                    ui.separator();
                    ui.label("Flocking Weights");

                    changed |= ui
                        .add(
                            egui::Slider::new(&mut bg.settings.separation_weight, 0.0..=5.0)
                                .text("Separation Weight"),
                        )
                        .changed();
                    changed |= ui
                        .add(
                            egui::Slider::new(&mut bg.settings.alignment_weight, 0.0..=5.0)
                                .text("Alignment Weight"),
                        )
                        .changed();
                    changed |= ui
                        .add(
                            egui::Slider::new(&mut bg.settings.cohesion_weight, 0.0..=5.0)
                                .text("Cohesion Weight"),
                        )
                        .changed();

                    if changed {
                        bg.apply_settings();
                    }
                }
                Graph::Life(bg) => {
                    let mut changed_visuals: bool = false;
                    let mut reset_board: bool = false;

                    ui.label("Game of Life Settings");

                    changed_visuals |= ui
                        .add(egui::Slider::new(&mut bg.settings.mesh_zoom, 0.1..=5.0).text("Zoom"))
                        .changed();

                    ui.separator();
                    ui.label("Simulation Rules");
                    changed_visuals |= ui
                        .add(
                            egui::Slider::new(&mut bg.settings.tick_rate, 0.01..=1.0)
                                .text("Tick Rate (Seconds)"),
                        )
                        .changed();

                    // If we change dimensions, flag it to reset the board
                    reset_board |= ui
                        .add(egui::Slider::new(&mut bg.settings.cols, 100..=500).text("Columns"))
                        .changed();
                    reset_board |= ui
                        .add(egui::Slider::new(&mut bg.settings.rows, 100..=500).text("Rows"))
                        .changed();
                    reset_board |= ui
                        .add(
                            egui::Slider::new(&mut bg.settings.initial_density, 0.01..=0.5)
                                .text("Start Density"),
                        )
                        .changed();

                    if ui.button("Regenerate Board").clicked() {
                        reset_board = true;
                    }

                    if changed_visuals || reset_board {
                        bg.apply_settings(reset_board);
                    }
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
            Self::Life(bg) => {
                bg.settings = LifeGraphSettings::default();
                bg.apply_settings(true);
            }
            Self::Boids(bg) => {
                bg.settings = BoidsGraphSettings::default();
                bg.apply_settings();
            }
        }
    }
}
