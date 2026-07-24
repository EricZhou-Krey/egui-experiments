use egui::{Pos2, Vec2};
use glam::USizeVec2;
use itertools::Itertools;
use shared_view::Viewable;

pub struct TriangulationGraph {
    width: f32,
    height: f32,

    n_points: usize,
    points: Vec<Pos2>,

    point_size: f32,
    point_color: egui::Color32,
    point_velocity: Vec<Vec2>,

    edge_length: f32,
    edge_color: egui::Color32,
    edges: Vec<USizeVec2>,
}

impl Default for TriangulationGraph {
    fn default() -> Self {
        Self {
            width: 0.0,
            height: 0.0,

            n_points: 100,
            points: Vec::with_capacity(100),

            point_size: 4.0,
            point_color: egui::Color32::RED,
            point_velocity: (0..100)
                .map(|_| Vec2 {
                    x: (rand::random::<f32>() - 0.5) * 0.5,
                    y: (rand::random::<f32>() - 0.5) * 0.5,
                })
                .collect(),

            edge_length: 2.0,
            edge_color: egui::Color32::LIGHT_RED,
            edges: Vec::with_capacity(100),
        }
    }
}

impl TriangulationGraph {
    pub fn re_initialize(&mut self, dimensions: Vec2) {
        self.width = dimensions.x;
        self.height = dimensions.y;
        self.points = (0..self.n_points)
            .map(|_| Pos2 {
                x: rand::random::<f32>() * dimensions.x,
                y: rand::random::<f32>() * dimensions.y,
            })
            .collect();

        self.update_edges();
    }

    fn update_points(&mut self) {
        for i in 0..self.points.len() {
            self.points[i] += self.point_velocity[i];
            self.points[i].x = self.points[i].x.rem_euclid(self.width);
            self.points[i].y = self.points[i].y.rem_euclid(self.height);
        }
    }

    fn delaunay_triangulation(
        points: &[Pos2],
        sorted_indicies: &[usize],
        left: usize,
        right: usize,
    ) -> Vec<USizeVec2> {
        if right - left < 3 {
            return sorted_indicies[left..=right]
                .iter()
                .permutations(2)
                .map(|p| USizeVec2 { x: *p[0], y: *p[1] })
                .collect();
        }

        let midpoint: usize = (left + right) / 2;
        let mut result: Vec<USizeVec2> =
            TriangulationGraph::delaunay_triangulation(points, sorted_indicies, left, midpoint);

        result.extend(TriangulationGraph::delaunay_triangulation(
            points,
            sorted_indicies,
            midpoint + 1,
            right,
        ));

        result
    }

    fn update_edges(&mut self) {
        let mut sorted_indicies: Vec<usize> = (0..self.points.len()).collect();
        sorted_indicies.sort_unstable_by(|&i, &j| {
            (self.points[i].x, self.points[i].y)
                .partial_cmp(&(self.points[j].x, self.points[j].y))
                .unwrap()
        });

        self.edges = TriangulationGraph::delaunay_triangulation(
            &self.points,
            &sorted_indicies,
            0,
            self.points.len() - 1,
        );
    }
}

impl Viewable for TriangulationGraph {
    fn title(&self) -> &str {
        "Detri Graph"
    }

    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        let full_size: Vec2 = ui.available_size();

        if full_size.x != self.width || full_size.y != self.height {
            self.re_initialize(full_size);
        }

        self.update_points();
        self.update_edges();

        let (response, painter) =
            ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());

        painter.rect_filled(response.rect, 0.0, egui::Color32::from_gray(30));

        let edge_style: egui::Stroke = egui::Stroke {
            width: self.edge_length,
            color: self.edge_color,
        };

        for edge in &self.edges {
            painter.line_segment([self.points[edge.x], self.points[edge.y]], edge_style);
        }

        for pos in self.points.clone() {
            painter.circle_filled(pos, self.point_size, self.point_color);
        }
    }
}

#[derive(Default)]
pub struct Navigator {
    triangulation_graph: TriangulationGraph,
}

impl Viewable for Navigator {
    fn title(&self) -> &str {
        "Navigator"
    }

    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        self.triangulation_graph.draw_ui(ui);
    }

    fn is_closeable(&self) -> bool {
        false
    }
}
