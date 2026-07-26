use shared_view::Viewable;
use std::collections::{HashMap, HashSet};

pub struct TriangulationGraph {
    dimensions: (f32, f32),

    camera_pos: (f32, f32, f32),
    camera_facing_direction: (f32, f32, f32),

    n_points: usize,
    points: Vec<(f32, f32)>,
    point_height: Vec<f32>,
    point_size: f32,
    point_color: egui::Color32,
    point_velocity: Vec<(f32, f32)>,

    edge_length: f32,
    edge_color: egui::Color32,
    edges: HashSet<(usize, usize)>,
}

impl Default for TriangulationGraph {
    fn default() -> Self {
        let n_points: usize = 200;
        Self {
            dimensions: (0.0, 0.0),
            camera_pos: (0.0, 0.0, -40.0),
            camera_facing_direction: (0.0, 0.0, 1.0),

            n_points,
            points: Vec::with_capacity(n_points),
            point_height: (0..n_points).map(|_| rand::random::<f32>()).collect(),
            point_size: 4.0,
            point_color: egui::Color32::RED,
            point_velocity: (0..n_points)
                .map(|_| {
                    (
                        (rand::random::<f32>() - 0.5) * 0.5,
                        (rand::random::<f32>() - 0.5) * 0.5,
                    )
                })
                .collect(),

            edge_length: 2.0,
            edge_color: egui::Color32::LIGHT_RED,
            edges: HashSet::with_capacity(3 * n_points),
        }
    }
}

struct TriangulationCircle {
    a: glam::Vec2,
    b: glam::Vec2,
    c: glam::Vec2,
}

impl TriangulationCircle {
    fn in_circle(&self, p: glam::Vec2) -> bool {
        let da = self.a - p;
        let db = self.b - p;
        let dc = self.c - p;

        glam::Mat3::from_cols(
            da.extend(da.length_squared()),
            db.extend(db.length_squared()),
            dc.extend(dc.length_squared()),
        )
        .determinant()
            > 0.0
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

    fn delaunay_triangulation(
        points: &[(f32, f32)],
        x_sorted_indicies: &[usize],
        left: usize,
        right: usize,
    ) -> HashSet<(usize, usize)> {
        let count = right - left + 1;

        if count <= 1 {
            return HashSet::new();
        }

        if count == 2 {
            return HashSet::from([(x_sorted_indicies[left], x_sorted_indicies[right])]);
        }

        if count == 3 {
            let a = x_sorted_indicies[left];
            let b = x_sorted_indicies[left + 1];
            let c = x_sorted_indicies[left + 2];

            let mut edges = HashSet::from([(a, b), (b, c)]);

            let a_p = glam::Vec2::from(points[a]);
            let b_p = glam::Vec2::from(points[b]);
            let c_p = glam::Vec2::from(points[c]);

            if (b_p - a_p).perp_dot(c_p - a_p).abs() > 1e-4 {
                edges.insert((c, a));
            }

            return edges;
        }

        let middle: usize = left + (right - left) / 2;
        let left_edges =
            TriangulationGraph::delaunay_triangulation(points, x_sorted_indicies, left, middle);
        let right_edges = TriangulationGraph::delaunay_triangulation(
            points,
            x_sorted_indicies,
            middle + 1,
            right,
        );

        TriangulationGraph::delaunay_merge(
            points,
            x_sorted_indicies,
            left_edges,
            right_edges,
            left,
            middle,
            right,
        )
    }

    fn delaunay_merge(
        points: &[(f32, f32)],
        x_sorted_indicies: &[usize],
        mut left_edges: HashSet<(usize, usize)>,
        mut right_edges: HashSet<(usize, usize)>,
        left: usize,
        middle: usize,
        right: usize,
    ) -> HashSet<(usize, usize)> {
        let mut middle_edges: HashSet<(usize, usize)> = HashSet::new();
        let get_p = |idx: usize| glam::vec2(points[idx].0, points[idx].1);

        let ccw = |a: glam::Vec2, b: glam::Vec2, c: glam::Vec2| -> f32 { (b - a).perp_dot(c - a) };

        let mut left_adj: HashMap<usize, Vec<usize>> = HashMap::new();
        for &(u, v) in &left_edges {
            left_adj.entry(u).or_default().push(v);
            left_adj.entry(v).or_default().push(u);
        }

        let mut right_adj: HashMap<usize, Vec<usize>> = HashMap::new();
        for &(u, v) in &right_edges {
            right_adj.entry(u).or_default().push(v);
            right_adj.entry(v).or_default().push(u);
        }

        let mut left_current = x_sorted_indicies[middle];
        let mut right_current = x_sorted_indicies[middle + 1];

        loop {
            let mut changed = false;

            for &n in &x_sorted_indicies[left..=middle] {
                if n == left_current {
                    continue;
                }
                if ccw(get_p(left_current), get_p(right_current), get_p(n)) < 0.0 {
                    left_current = n;
                    changed = true;
                }
            }

            for &n in &x_sorted_indicies[middle + 1..=right] {
                if n == right_current {
                    continue;
                }
                if ccw(get_p(left_current), get_p(right_current), get_p(n)) < 0.0 {
                    right_current = n;
                    changed = true;
                }
            }

            if !changed {
                break;
            }
        }

        loop {
            middle_edges.insert((left_current, right_current));

            let mut right_candidate = None;
            if let Some(neighbors) = right_adj.get(&right_current) {
                let mut valid_neighbors = Vec::new();
                for &n in neighbors {
                    if !right_edges.contains(&(right_current, n))
                        && !right_edges.contains(&(n, right_current))
                    {
                        continue;
                    }
                    if ccw(get_p(left_current), get_p(right_current), get_p(n)) > 0.0 {
                        valid_neighbors.push(n);
                    }
                }

                let base_ray = get_p(left_current) - get_p(right_current);
                valid_neighbors.sort_by(|&a, &b| {
                    let ray_a = get_p(a) - get_p(right_current);
                    let ray_b = get_p(b) - get_p(right_current);
                    let angle_a = base_ray.perp_dot(ray_a).atan2(base_ray.dot(ray_a));
                    let angle_b = base_ray.perp_dot(ray_b).atan2(base_ray.dot(ray_b));
                    angle_b.partial_cmp(&angle_a).unwrap()
                });

                let mut i = 0;
                while i < valid_neighbors.len() {
                    if i + 1 < valid_neighbors.len() {
                        let c = TriangulationCircle {
                            a: get_p(left_current),
                            b: get_p(right_current),
                            c: get_p(valid_neighbors[i]),
                        };
                        if c.in_circle(get_p(valid_neighbors[i + 1])) {
                            right_edges.remove(&(right_current, valid_neighbors[i]));
                            right_edges.remove(&(valid_neighbors[i], right_current));
                            i += 1;
                            continue;
                        }
                    }
                    right_candidate = Some(valid_neighbors[i]);
                    break;
                }
            }

            let mut left_candidate = None;
            if let Some(neighbors) = left_adj.get(&left_current) {
                let mut valid_neighbors = Vec::new();
                for &n in neighbors {
                    if !left_edges.contains(&(left_current, n))
                        && !left_edges.contains(&(n, left_current))
                    {
                        continue;
                    }
                    if ccw(get_p(left_current), get_p(right_current), get_p(n)) > 0.0 {
                        valid_neighbors.push(n);
                    }
                }

                let base_ray = get_p(right_current) - get_p(left_current);
                valid_neighbors.sort_by(|&a, &b| {
                    let ray_a = get_p(a) - get_p(left_current);
                    let ray_b = get_p(b) - get_p(left_current);
                    let angle_a = base_ray.perp_dot(ray_a).atan2(base_ray.dot(ray_a));
                    let angle_b = base_ray.perp_dot(ray_b).atan2(base_ray.dot(ray_b));
                    angle_a.partial_cmp(&angle_b).unwrap()
                });

                let mut i = 0;
                while i < valid_neighbors.len() {
                    if i + 1 < valid_neighbors.len() {
                        let c = TriangulationCircle {
                            a: get_p(left_current),
                            b: get_p(right_current),
                            c: get_p(valid_neighbors[i]),
                        };
                        if c.in_circle(get_p(valid_neighbors[i + 1])) {
                            left_edges.remove(&(left_current, valid_neighbors[i]));
                            left_edges.remove(&(valid_neighbors[i], left_current));
                            i += 1;
                            continue;
                        }
                    }
                    left_candidate = Some(valid_neighbors[i]);
                    break;
                }
            }

            let is_choosing_right = match (left_candidate, right_candidate) {
                (None, None) => break,
                (Some(_), None) => false,
                (None, Some(_)) => true,
                (Some(lc), Some(rc)) => {
                    let circle = TriangulationCircle {
                        a: get_p(left_current),
                        b: get_p(right_current),
                        c: get_p(lc),
                    };
                    circle.in_circle(get_p(rc))
                }
            };

            if is_choosing_right {
                right_current = right_candidate.unwrap();
            } else {
                left_current = left_candidate.unwrap();
            }
        }

        let mut edges: HashSet<(usize, usize)> = middle_edges;
        edges.extend(left_edges);
        edges.extend(right_edges);
        edges
    }

    fn update_edges(&mut self) {
        let mut x_sorted_indicies: Vec<usize> = (0..self.points.len()).collect();
        x_sorted_indicies
            .sort_unstable_by(|&i, &j| self.points[i].partial_cmp(&self.points[j]).unwrap());

        self.edges = TriangulationGraph::delaunay_triangulation(
            &self.points,
            &x_sorted_indicies,
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
