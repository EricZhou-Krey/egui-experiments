pub mod boids;
pub mod life;
pub mod triangulation;

use egui::{Pos2, Rect};
use glam::Vec2;

use crate::{
    graph::boids::graph::BoidGraph,
    graph::life::graph::LifeGraph,
    graph::triangulation::graph::TriangulationGraph,
    layouts::Layout,
    settings::{InteractableTriangulationMeshSettings, TriangulationGraphSettings},
};

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
                // todo!();
                None
            }
            Self::Boids(_bg) => {
                // todo!();
                None
            }
        }
    }

    pub fn set_interact_index(&mut self, index: Option<GraphInteractNodeIndex>) {
        match self {
            Self::Triangulation(bg) => bg.mesh.interact_vertex = index,
            Self::Life(_bg) => {
                //todo!();
            }
            Self::Boids(_bg) => {
                //todo!();
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
                //todo!()
            }
            Self::Boids(_bg) => {
                //todo!()
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
