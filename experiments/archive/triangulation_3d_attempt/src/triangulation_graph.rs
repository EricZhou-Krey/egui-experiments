use crate::space_renderer::{RenderPrimitive, SpaceRenderer};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub struct TriangulationGraphSettings {
    dimensions: (f32, f32, f32),
    screen_origin: (f32, f32),

    camera_origin: (f32, f32, f32),
    camera_default_direction: (f32, f32, f32),
    camera_move_speed: f32,
    camera_rotation_speed: f32,

    aspect_ratio: f32,
    z_near: f32,
    z_far: f32,
    fov_y: f32,

    pub n_points: usize,
    point_size: f32,
    point_color: egui::Color32,

    edge_width: f32,
    edge_color: egui::Color32,

    face_color: egui::Color32,
}

impl Default for TriangulationGraphSettings {
    fn default() -> Self {
        Self {
            dimensions: (0.0, 0.0, 200.0),
            screen_origin: (0.0, 0.0),

            camera_origin: (0.0, 0.0, 0.0),
            camera_default_direction: (0.0, 0.0, 1.0),
            camera_move_speed: 5.0,
            camera_rotation_speed: 0.01,

            aspect_ratio: 1.0,
            z_near: 0.1,
            z_far: 1000.0,
            fov_y: 60.0_f32.to_radians(),

            n_points: 200,
            point_size: 4.0,
            point_color: egui::Color32::RED,

            edge_width: 2.0,
            edge_color: egui::Color32::LIGHT_RED,

            face_color: egui::Color32::DARK_RED,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TriangulationGraph {
    pub settings: TriangulationGraphSettings,

    camera_pos: (f32, f32, f32),
    camera_facing_direction: (f32, f32, f32),

    points: Vec<(f32, f32, f32)>,
    point_velocity: Vec<(f32, f32, f32)>,

    edges: HashSet<(usize, usize)>,

    pub screen_points: Vec<((f32, f32), f32)>,
}

impl TriangulationGraph {
    pub fn new() -> Self {
        let settings: TriangulationGraphSettings = TriangulationGraphSettings::default();
        let n_points = settings.n_points;
        let camera_pos = settings.camera_origin;
        let camera_facing_direction = settings.camera_default_direction;
        Self {
            settings,

            camera_pos,
            camera_facing_direction,

            points: Vec::with_capacity(n_points),
            point_velocity: (0..n_points)
                .map(|_| {
                    (
                        (rand::random::<f32>() - 0.5) * 0.5,
                        (rand::random::<f32>() - 0.5) * 0.5,
                        0.0,
                    )
                })
                .collect(),

            edges: HashSet::with_capacity(3 * n_points),

            screen_points: Vec::with_capacity(n_points),
        }
    }

    pub fn re_initialize(&mut self, mut settings: TriangulationGraphSettings) {
        self.camera_pos = settings.camera_origin;

        settings.aspect_ratio = if settings.dimensions.1 > 0.0 {
            settings.dimensions.0 / settings.dimensions.1
        } else {
            1.0
        };

        self.points = (0..settings.n_points)
            .map(|_| {
                (
                    rand::random::<f32>() * settings.dimensions.0,
                    rand::random::<f32>() * settings.dimensions.1,
                    rand::random::<f32>() * settings.dimensions.2,
                )
            })
            .collect();

        self.settings = settings;
        self.update_edges();
    }

    fn update_points(&mut self) {
        for i in 0..self.points.len() {
            self.points[i] = (glam::Vec3::from(self.points[i])
                + glam::Vec3::from(self.point_velocity[i]))
            .into();

            self.points[i] = (
                self.points[i].0.rem_euclid(self.settings.dimensions.0),
                self.points[i].1.rem_euclid(self.settings.dimensions.1),
                self.points[i].2.rem_euclid(self.settings.dimensions.2),
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

        let xy_points: Vec<(f32, f32)> = self.points.iter().map(|&(x, y, _)| (x, y)).collect();

        self.edges = TriangulationGraph::delaunay_triangulation(
            &xy_points,
            &x_sorted_indicies,
            0,
            self.points.len() - 1,
        );
    }

    pub fn update_renderer(&mut self, renderer: &mut SpaceRenderer) {
        let cam_pos: glam::Vec3 = glam::Vec3::from(self.camera_pos);
        let cam_dir: glam::Vec3 =
            glam::Vec3::from(self.camera_facing_direction).normalize_or_zero();
        let up_vector: glam::Vec3 = glam::Vec3::Y;

        let view_mat: glam::Mat4 =
            glam::camera::rh::view::look_at_mat4(cam_pos, cam_pos + cam_dir, up_vector);
        let proj_mat: glam::Mat4 = glam::camera::rh::proj::directx::perspective(
            self.settings.fov_y,
            self.settings.aspect_ratio,
            self.settings.z_near,
            self.settings.z_far,
        );

        let view_proj_mat: glam::Mat4 = proj_mat * view_mat;

        self.screen_points.clear();

        for &(x, y, z) in self.points.iter() {
            let world_pos: glam::Vec4 = glam::vec4(x, y, z, 1.0);
            let clip_space_pos: glam::Vec4 = view_proj_mat * world_pos;

            if clip_space_pos.w <= 0.0 {
                continue;
            }

            let ndc_x = clip_space_pos.x / clip_space_pos.w;
            let ndc_y = clip_space_pos.y / clip_space_pos.w;

            let screen_x = (ndc_x + 1.0) * 0.5 * self.settings.dimensions.0;
            let screen_y = (1.0 - ndc_y) * 0.5 * self.settings.dimensions.1;

            self.screen_points.push((
                (
                    screen_x + self.settings.screen_origin.0,
                    screen_y + self.settings.screen_origin.1,
                ),
                clip_space_pos.w,
            ));
        }

        for &(point, depth) in self.screen_points.iter() {
            renderer.primitives_buffer.push(RenderPrimitive::Point {
                point: point.into(),
                depth,
                radius: self.settings.point_size,
                color: self.settings.point_color,
            });
        }

        let stroke = egui::Stroke::new(self.settings.edge_width, self.settings.edge_color);

        for &(u, v) in &self.edges {
            let ((p1_pos, p1_depth), (p2_pos, p2_depth)) =
                (&self.screen_points[u], &self.screen_points[v]);
            renderer.primitives_buffer.push(RenderPrimitive::Edge {
                pts: [(*p1_pos).into(), (*p2_pos).into()],
                depth: (p1_depth + p2_depth) / 2.0,
                stroke,
            });
        }

        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); self.points.len()];
        for &(u, v) in &self.edges {
            adj[u].push(v);
            adj[v].push(u);
        }

        let has_edge =
            |u: usize, v: usize| self.edges.contains(&(u, v)) || self.edges.contains(&(v, u));

        for i in 0..self.points.len() {
            for &j in &adj[i] {
                if j > i {
                    for &k in &adj[j] {
                        if k > j && has_edge(i, k) {
                            let ((pi_pos, pi_depth), (pj_pos, pj_depth), (pk_pos, pk_depth)) = (
                                &self.screen_points[i],
                                &self.screen_points[j],
                                &self.screen_points[k],
                            );

                            let p_i: glam::Vec2 = (*pi_pos).into();
                            let p_j: glam::Vec2 = (*pj_pos).into();
                            let p_k: glam::Vec2 = (*pk_pos).into();

                            let v1 = p_j - p_i;
                            let v2 = p_k - p_i;
                            let v3 = p_k - p_j;

                            let cross = v1.perp_dot(v2).abs();

                            let max_edge_sq = v1
                                .length_squared()
                                .max(v2.length_squared())
                                .max(v3.length_squared());

                            if cross * cross > max_edge_sq {
                                let avg_depth = (pi_depth + pj_depth + pk_depth) / 3.0;
                                renderer.primitives_buffer.push(RenderPrimitive::Face {
                                    pts: [(*pi_pos).into(), (*pj_pos).into(), (*pk_pos).into()],
                                    depth: avg_depth,
                                    face_color: self.settings.face_color,
                                    stroke: egui::Stroke::NONE,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    fn handle_input(&mut self, ui: &mut egui::Ui) {
        let mut pos = glam::Vec3::from(self.camera_pos);
        let mut dir = glam::Vec3::from(self.camera_facing_direction).normalize_or_zero();

        let up = glam::Vec3::Y;
        let right = dir.cross(up).normalize_or_zero();

        let mut input_velocity = glam::Vec3::ZERO;
        let mut input_rotation = glam::Vec2::ZERO;
        let mut reset: bool = false;

        ui.input(|i| {
            for key in &i.keys_down {
                match key {
                    egui::Key::R => reset = true,
                    egui::Key::W => input_velocity += dir,
                    egui::Key::S => input_velocity -= dir,
                    egui::Key::D => input_velocity += right,
                    egui::Key::A => input_velocity -= right,
                    egui::Key::E => input_velocity += up,
                    egui::Key::Q => input_velocity -= up,
                    egui::Key::ArrowLeft => input_rotation.y += 1.0,
                    egui::Key::ArrowRight => input_rotation.y -= 1.0,
                    egui::Key::ArrowUp => input_rotation.x += 1.0,
                    egui::Key::ArrowDown => input_rotation.x -= 1.0,
                    _ => {}
                }
            }
        });

        pos += input_velocity.normalize_or_zero() * self.settings.camera_move_speed;

        let yaw =
            glam::Quat::from_axis_angle(up, input_rotation.y * self.settings.camera_rotation_speed);
        let pitch = glam::Quat::from_axis_angle(
            right,
            input_rotation.x * self.settings.camera_rotation_speed,
        );
        dir = (yaw * pitch) * dir;

        if reset {
            self.camera_pos = self.settings.camera_origin;
            self.camera_facing_direction = self.settings.camera_default_direction;
        } else {
            self.camera_pos = pos.into();
            self.camera_facing_direction = dir.into();
        }
    }

    pub fn draw_ui(&mut self, ui: &mut egui::Ui) {
        let full_size: egui::Vec2 = ui.available_size();
        let screen_origin: egui::Pos2 = ui.available_rect_before_wrap().min;

        // Inefficent check, could be refactored
        if full_size.x != self.settings.dimensions.0 || full_size.y != self.settings.dimensions.1 {
            let settings: TriangulationGraphSettings = TriangulationGraphSettings {
                dimensions: (full_size.x, full_size.y, self.settings.dimensions.2),
                screen_origin: screen_origin.into(),
                camera_origin: (full_size.x / 2.0, full_size.y / 2.0, -800.0),
                ..Default::default()
            };

            self.re_initialize(settings);
        }

        self.update_points();
        self.update_edges();
        self.handle_input(ui);
    }
}
