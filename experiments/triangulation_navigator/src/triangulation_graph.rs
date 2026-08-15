use std::ops::{Deref, DerefMut};

use crate::{
    style::{FaceStyle, GraphStyle, LineStyle, PointStyle},
    triangulation_mesh::{AnimatedTriangulationMesh, HalfEdge},
};
use egui::{Color32, Painter, Pos2, Rect, Shape, Stroke, Ui};
use shared_view::viewable::Viewable;

#[derive(Debug, Clone)]
pub struct InteractableTriangulationMeshSettings {
    pub n_points: usize,
    pub point_speed: f32,
}

impl Default for InteractableTriangulationMeshSettings {
    fn default() -> Self {
        Self {
            n_points: 200,
            point_speed: 0.05,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct InteractableTriangulationMesh {
    pub animated_mesh: AnimatedTriangulationMesh,
    // pub highlighted_vertices: Vec<usize>,
}

impl Deref for InteractableTriangulationMesh {
    type Target = AnimatedTriangulationMesh;

    fn deref(&self) -> &Self::Target {
        &self.animated_mesh
    }
}

impl DerefMut for InteractableTriangulationMesh {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animated_mesh
    }
}

impl InteractableTriangulationMesh {
    fn new(settings: InteractableTriangulationMeshSettings) -> Self {
        InteractableTriangulationMesh {
            animated_mesh: AnimatedTriangulationMesh::new(settings.n_points, settings.point_speed),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TriangulationGraphSettings {
    pub mesh_zoom: f32,
}

impl Default for TriangulationGraphSettings {
    fn default() -> Self {
        Self { mesh_zoom: 1.15 }
    }
}

#[derive(Debug, Clone)]
pub struct TriangulationGraph {
    pub mesh: InteractableTriangulationMesh,
    pub settings: TriangulationGraphSettings,
    pub style: GraphStyle,
}

impl Default for TriangulationGraph {
    fn default() -> Self {
        Self::new(
            TriangulationGraphSettings::default(),
            InteractableTriangulationMeshSettings::default(),
        )
    }
}

impl TriangulationGraph {
    pub fn new(
        graph_settings: TriangulationGraphSettings,
        mesh_settings: InteractableTriangulationMeshSettings,
    ) -> Self {
        Self {
            mesh: InteractableTriangulationMesh::new(mesh_settings),
            settings: graph_settings,
            style: GraphStyle {
                point: PointStyle {
                    radius: 2.0,
                    color: Color32::WHITE,
                },
                line: LineStyle {
                    stroke: Stroke::new(1.5, Color32::LIGHT_GRAY),
                },
                face: FaceStyle {
                    fill_color: Color32::from_rgb(40, 44, 52),
                    border_stroke: Stroke::NONE,
                },
            },
        }
    }
}

impl Viewable for TriangulationGraph {
    fn draw_ui(&mut self, ui: &mut Ui) {
        let dt: f32 = ui.input(|i| i.stable_dt).min(0.1);

        let rect: Rect = ui.available_rect_before_wrap();

        let base_scale: f32 = rect.width().max(rect.height());
        let base_offset_x: f32 = (rect.width() - base_scale) / 2.0;
        let base_offset_y: f32 = (rect.height() - base_scale) / 2.0;

        let bounds: [f32; 4] = [
            -base_offset_x / base_scale,
            (rect.width() - base_offset_x) / base_scale,
            -base_offset_y / base_scale,
            (rect.height() - base_offset_y) / base_scale,
        ];

        self.mesh.update(dt, bounds);

        ui.request_repaint();

        let painter: Painter = ui.painter().with_clip_rect(rect);

        let render_scale: f32 = base_scale * self.settings.mesh_zoom;
        let render_offset_x: f32 = (rect.width() - render_scale) / 2.0;
        let render_offset_y: f32 = (rect.height() - render_scale) / 2.0;

        let to_screen = |pos: [f32; 2]| -> Pos2 {
            Pos2::new(
                rect.min.x + render_offset_x + (pos[0] * render_scale),
                rect.min.y + render_offset_y + (pos[1] * render_scale),
            )
        };

        for face in self.mesh.mesh.faces.iter() {
            let mut points: Vec<Pos2> = Vec::new();
            let mut current_half_edge: &HalfEdge = &self.mesh.mesh.half_edges[face.edge];

            for _ in 0..3 {
                let raw_pos: [f32; 2] = self.mesh.mesh.vertices[current_half_edge.origin].pos;
                points.push(to_screen(raw_pos));
                current_half_edge = &self.mesh.mesh.half_edges[current_half_edge.next];
            }

            painter.add(Shape::convex_polygon(
                points,
                self.style.face.fill_color,
                self.style.face.border_stroke,
            ));
        }

        for (edge_index, edge) in self.mesh.mesh.half_edges.iter().enumerate() {
            if edge_index < edge.twin.unwrap_or(usize::MAX) {
                let p1: Pos2 = to_screen(self.mesh.mesh.vertices[edge.origin].pos);
                let p2: Pos2 = to_screen(
                    self.mesh.mesh.vertices[self.mesh.mesh.half_edges[edge.next].origin].pos,
                );

                painter.line_segment([p1, p2], self.style.line.stroke);
            }
        }
    }
}
