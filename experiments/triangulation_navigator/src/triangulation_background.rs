use crate::triangulation_mesh::{HalfEdge, TriangulationMesh};
use shared_view::viewable::Viewable;

#[derive(Debug, Default, Clone)]
pub struct TriangulationBackground {
    pub mesh: TriangulationMesh,
}

impl TriangulationBackground {
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
        }
    }
}

impl Viewable for TriangulationBackground {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        let painter: &egui::Painter = ui.painter();
        for face in self.mesh.faces.iter() {
            let mut points: Vec<egui::Pos2> = Vec::new();
            let mut current_half_edge: &HalfEdge = &self.mesh.half_edges[face.edge];

            for _ in 0..3 {
                points.push(self.mesh.vertices[current_half_edge.origin].pos.into());

                current_half_edge = &self.mesh.half_edges[current_half_edge.next];
            }

            painter.add(egui::Shape::convex_polygon(
                points,
                egui::Color32::DARK_GRAY,
                egui::Stroke::NONE,
            ));
        }

        for (edge_index, edge) in self.mesh.half_edges.iter().enumerate() {
            if edge_index < edge.twin.unwrap_or(usize::MAX) {
                let p1: egui::Pos2 = self.mesh.vertices[edge.origin].pos.into();
                let p2: egui::Pos2 = self.mesh.vertices[self.mesh.half_edges[edge.next].origin]
                    .pos
                    .into();

                painter.line_segment([p1, p2], egui::Stroke::new(1.0, egui::Color32::BLACK));
            }
        }
    }
}
