use crate::{
    style::{FaceStyle, GraphStyle, LineStyle, PointStyle},
    triangulation_mesh::{AnimatedTriangulationMesh, HalfEdge},
};
use egui::{Color32, Painter, Pos2, Rect, Shape, Stroke, Ui};
use shared_view::viewable::Viewable;

#[derive(Debug, Clone, Default)]
pub struct TriangulationGraph {
    pub animated_mesh: AnimatedTriangulationMesh,
    pub style: GraphStyle,
}

impl TriangulationGraph {
    pub fn new(n_points: usize) -> Self {
        Self {
            animated_mesh: AnimatedTriangulationMesh::new(n_points, 0.01),
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

        let scale: f32 = rect.width().max(rect.height());
        let offset_x: f32 = (rect.width() - scale) / 2.0;
        let offset_y: f32 = (rect.height() - scale) / 2.0;

        let bounds: [f32; 4] = [
            -offset_x / scale,
            (rect.width() - offset_x) / scale,
            -offset_y / scale,
            (rect.height() - offset_y) / scale,
        ];

        self.animated_mesh.update(dt, bounds);

        ui.request_repaint();

        let painter: &Painter = ui.painter();

        let to_screen = |pos: [f32; 2]| -> Pos2 {
            Pos2::new(
                rect.min.x + offset_x + (pos[0] * scale),
                rect.min.y + offset_y + (pos[1] * scale),
            )
        };

        for face in self.animated_mesh.mesh.faces.iter() {
            let mut points: Vec<Pos2> = Vec::new();
            let mut current_half_edge: &HalfEdge = &self.animated_mesh.mesh.half_edges[face.edge];

            for _ in 0..3 {
                let raw_pos: [f32; 2] =
                    self.animated_mesh.mesh.vertices[current_half_edge.origin].pos;
                points.push(to_screen(raw_pos));
                current_half_edge = &self.animated_mesh.mesh.half_edges[current_half_edge.next];
            }

            painter.add(Shape::convex_polygon(
                points,
                self.style.face.fill_color,
                self.style.face.border_stroke,
            ));
        }

        for (edge_index, edge) in self.animated_mesh.mesh.half_edges.iter().enumerate() {
            if edge_index < edge.twin.unwrap_or(usize::MAX) {
                let p1: Pos2 = to_screen(self.animated_mesh.mesh.vertices[edge.origin].pos);
                let p2: Pos2 = to_screen(
                    self.animated_mesh.mesh.vertices
                        [self.animated_mesh.mesh.half_edges[edge.next].origin]
                        .pos,
                );

                painter.line_segment([p1, p2], self.style.line.stroke);
            }
        }
    }
}
