use crate::{
    style::{FaceStyle, GraphStyle, LineStyle, PointStyle},
    triangulation_mesh::{HalfEdge, TriangulationMesh},
};
use egui::{Color32, Painter, Pos2, Shape, Stroke, Ui};
use shared_view::viewable::Viewable;

#[derive(Debug, Default, Clone)]
pub struct TriangulationGraph {
    pub mesh: TriangulationMesh,
    pub style: GraphStyle,
}

impl TriangulationGraph {
    pub fn new() -> Self {
        let points: Vec<[f32; 2]> = std::iter::repeat_with(|| {
            [
                rand::random::<f32>() * 1000.0,
                rand::random::<f32>() * 1000.0,
            ]
        })
        .take(200)
        .collect();

        Self {
            mesh: TriangulationMesh::from_points(&points),
            style: GraphStyle {
                point: PointStyle {
                    radius: 4.0,
                    color: Color32::WHITE,
                },
                line: LineStyle {
                    stroke: Stroke::new(1.5, Color32::LIGHT_GRAY),
                },
                face: FaceStyle {
                    fill_color: Color32::from_rgb(40, 44, 52),
                    border_stroke: Stroke::new(1.0, Color32::GRAY),
                },
            },
        }
    }
}

impl Viewable for TriangulationGraph {
    fn draw_ui(&mut self, ui: &mut Ui) {
        let painter: &Painter = ui.painter();
        for face in self.mesh.faces.iter() {
            let mut points: Vec<Pos2> = Vec::new();
            let mut current_half_edge: &HalfEdge = &self.mesh.half_edges[face.edge];

            for _ in 0..3 {
                points.push(self.mesh.vertices[current_half_edge.origin].pos.into());

                current_half_edge = &self.mesh.half_edges[current_half_edge.next];
            }

            painter.add(Shape::convex_polygon(
                points,
                self.style.face.fill_color,
                self.style.face.border_stroke,
            ));
        }

        for (edge_index, edge) in self.mesh.half_edges.iter().enumerate() {
            if edge_index < edge.twin.unwrap_or(usize::MAX) {
                let p1: Pos2 = self.mesh.vertices[edge.origin].pos.into();
                let p2: Pos2 = self.mesh.vertices[self.mesh.half_edges[edge.next].origin]
                    .pos
                    .into();

                painter.line_segment([p1, p2], self.style.line.stroke);
            }
        }
    }
}
