use itertools::Itertools;
use shared_view::Viewable;
use std::collections::HashSet;

pub struct TriangulationGraph {
    dimensions: (f32, f32),

    n_points: usize,
    points: Vec<(f32, f32)>,

    point_size: f32,
    point_color: egui::Color32,
    point_velocity: Vec<(f32, f32)>,

    edge_length: f32,
    edge_color: egui::Color32,
    edges: HashSet<(usize, usize)>,
}

impl Default for TriangulationGraph {
    fn default() -> Self {
        Self {
            dimensions: (0.0, 0.0),

            n_points: 100,
            points: Vec::with_capacity(100),

            point_size: 4.0,
            point_color: egui::Color32::RED,
            point_velocity: (0..100)
                .map(|_| {
                    (
                        (rand::random::<f32>() - 0.5) * 0.5,
                        (rand::random::<f32>() - 0.5) * 0.5,
                    )
                })
                .collect(),

            edge_length: 2.0,
            edge_color: egui::Color32::LIGHT_RED,
            edges: HashSet::with_capacity(100),
        }
    }
}

impl TriangulationGraph {
    pub fn re_initialize(&mut self, dimensions: (f32, f32)) {
        self.dimensions = dimensions;
        self.points = (0..self.n_points)
            .map(|_| {
                (
                    rand::random::<f32>() * dimensions.0,
                    rand::random::<f32>() * dimensions.1,
                )
            })
            .collect();

        self.update_edges();
    }

    fn update_points(&mut self) {
        for i in 0..self.points.len() {
            self.points[i] = (glam::Vec2::from(self.points[i])
                + glam::Vec2::from(self.point_velocity[i]))
            .into();

            self.points[i] = (
                self.points[i].0.rem_euclid(self.dimensions.0),
                self.points[i].1.rem_euclid(self.dimensions.1),
            );
        }
    }

    fn delaunay_merge(
        points: &[(f32, f32)],
        mut left: HashSet<(usize, usize)>,
        right: HashSet<(usize, usize)>,
    ) -> HashSet<(usize, usize)> {
        left.extend(right);
        left
    }

    fn delaunay_triangulation(
        points: &[(f32, f32)],
        sorted_indicies: &[usize],
        left: usize,
        right: usize,
    ) -> HashSet<(usize, usize)> {
        if right - left < 3 {
            let collinear = {
                let v1: glam::Vec2 = glam::Vec2::from(points[sorted_indicies[right]])
                    - glam::Vec2::from(points[sorted_indicies[left]]);
                let v2: glam::Vec2 = glam::Vec2::from(points[sorted_indicies[right - 1]])
                    - glam::Vec2::from(points[sorted_indicies[left]]);

                right - left == 2 && v1.perp_dot(v2).abs() <= f32::EPSILON
            };

            if collinear {
                return HashSet::from([(sorted_indicies[left], sorted_indicies[right])]);
            } else {
                return sorted_indicies[left..=right]
                    .iter()
                    .permutations(2)
                    .map(|p| (*p[0], *p[1]))
                    .collect();
            }
        }

        let midpoint: usize = (left + right) / 2;
        let left: HashSet<(usize, usize)> =
            TriangulationGraph::delaunay_triangulation(points, sorted_indicies, left, midpoint);
        let right: HashSet<(usize, usize)> = TriangulationGraph::delaunay_triangulation(
            points,
            sorted_indicies,
            midpoint + 1,
            right,
        );

        TriangulationGraph::delaunay_merge(points, left, right)
    }

    fn update_edges(&mut self) {
        let mut sorted_indicies: Vec<usize> = (0..self.points.len()).collect();
        sorted_indicies
            .sort_unstable_by(|&i, &j| self.points[i].partial_cmp(&self.points[j]).unwrap());

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
        let full_size: egui::Vec2 = ui.available_size();

        if full_size.x != self.dimensions.0 || full_size.y != self.dimensions.1 {
            self.re_initialize(full_size.into());
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
            painter.line_segment(
                [
                    egui::Pos2::from(self.points[edge.0]),
                    egui::Pos2::from(self.points[edge.1]),
                ],
                edge_style,
            );
        }

        for pos in self.points.clone() {
            painter.circle_filled(egui::Pos2::from(pos), self.point_size, self.point_color);
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
