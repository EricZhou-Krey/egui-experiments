use glam::{vec2, Mat3, Vec2};
use std::collections::{HashMap, HashSet};

fn in_circle(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> bool {
    let da: Vec2 = a - d;
    let db: Vec2 = b - d;
    let dc: Vec2 = c - d;

    Mat3::from_cols(
        da.extend(da.length_squared()),
        db.extend(db.length_squared()),
        dc.extend(dc.length_squared()),
    )
    .determinant()
        > 0.0
}

pub type VertexIndex = usize;
pub type HalfEdgeIndex = usize;
pub type FaceIndex = usize;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub incident_edge: Option<HalfEdgeIndex>,
}

#[derive(Debug, Clone)]
pub struct HalfEdge {
    pub origin: VertexIndex,
    pub twin: Option<HalfEdgeIndex>,
    pub next: HalfEdgeIndex,
    pub face: FaceIndex,
}

#[derive(Debug, Clone)]
pub struct Face {
    pub edge: HalfEdgeIndex,
}

#[derive(Debug, Default, Clone)]
pub struct TriangulationMesh {
    pub vertices: Vec<Vertex>,
    pub half_edges: Vec<HalfEdge>,
    pub faces: Vec<Face>,
}

impl TriangulationMesh {
    fn triangulation(
        points: &[[f32; 2]],
        x_sorted_indicies: &[usize],
        left: usize,
        right: usize,
    ) -> HashSet<(usize, usize)> {
        let count: usize = right - left + 1;

        if count <= 1 {
            return HashSet::new();
        }

        if count == 2 {
            return HashSet::from([(x_sorted_indicies[left], x_sorted_indicies[right])]);
        }

        if count == 3 {
            let a: usize = x_sorted_indicies[left];
            let b: usize = x_sorted_indicies[left + 1];
            let c: usize = x_sorted_indicies[left + 2];

            let mut edges: HashSet<(usize, usize)> = HashSet::from([(a, b), (b, c)]);

            let a_p: Vec2 = Vec2::from(points[a]);
            let b_p: Vec2 = Vec2::from(points[b]);
            let c_p: Vec2 = Vec2::from(points[c]);

            if (b_p - a_p).perp_dot(c_p - a_p).abs() > 1e-4 {
                edges.insert((c, a));
            }

            return edges;
        }

        let middle: usize = left + (right - left) / 2;
        let left_edges: HashSet<(usize, usize)> =
            TriangulationMesh::triangulation(points, x_sorted_indicies, left, middle);
        let right_edges: HashSet<(usize, usize)> =
            TriangulationMesh::triangulation(points, x_sorted_indicies, middle + 1, right);

        TriangulationMesh::triangulation_merge(
            points,
            x_sorted_indicies,
            left_edges,
            right_edges,
            left,
            middle,
            right,
        )
    }

    fn triangulation_merge(
        points: &[[f32; 2]],
        x_sorted_indicies: &[usize],
        mut left_edges: HashSet<(usize, usize)>,
        mut right_edges: HashSet<(usize, usize)>,
        left: usize,
        middle: usize,
        right: usize,
    ) -> HashSet<(usize, usize)> {
        struct TriangulationCircle {
            a: Vec2,
            b: Vec2,
            c: Vec2,
        }

        impl TriangulationCircle {
            fn in_circle(&self, p: Vec2) -> bool {
                let da: Vec2 = self.a - p;
                let db: Vec2 = self.b - p;
                let dc: Vec2 = self.c - p;

                Mat3::from_cols(
                    da.extend(da.length_squared()),
                    db.extend(db.length_squared()),
                    dc.extend(dc.length_squared()),
                )
                .determinant()
                    > 0.0
            }
        }

        let mut middle_edges: HashSet<(usize, usize)> = HashSet::new();
        let get_point = |index: usize| -> Vec2 { vec2(points[index][0], points[index][1]) };

        let counter_clockwise = |a: Vec2, b: Vec2, c: Vec2| -> f32 { (b - a).perp_dot(c - a) };

        let mut left_adjacency: HashMap<usize, Vec<usize>> = HashMap::new();
        for &(u, v) in &left_edges {
            left_adjacency.entry(u).or_default().push(v);
            left_adjacency.entry(v).or_default().push(u);
        }

        let mut right_adjacency: HashMap<usize, Vec<usize>> = HashMap::new();
        for &(u, v) in &right_edges {
            right_adjacency.entry(u).or_default().push(v);
            right_adjacency.entry(v).or_default().push(u);
        }

        let mut left_current: usize = x_sorted_indicies[middle];
        let mut right_current: usize = x_sorted_indicies[middle + 1];

        loop {
            let mut changed: bool = false;

            for &n in &x_sorted_indicies[left..=middle] {
                if n == left_current {
                    continue;
                }
                if counter_clockwise(
                    get_point(left_current),
                    get_point(right_current),
                    get_point(n),
                ) < 0.0
                {
                    left_current = n;
                    changed = true;
                }
            }

            for &n in &x_sorted_indicies[middle + 1..=right] {
                if n == right_current {
                    continue;
                }
                if counter_clockwise(
                    get_point(left_current),
                    get_point(right_current),
                    get_point(n),
                ) < 0.0
                {
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

            let mut right_candidate: Option<usize> = None;
            if let Some(neighbors) = right_adjacency.get(&right_current) {
                let mut valid_neighbors: Vec<usize> = Vec::new();
                for &n in neighbors {
                    if !right_edges.contains(&(right_current, n))
                        && !right_edges.contains(&(n, right_current))
                    {
                        continue;
                    }
                    if counter_clockwise(
                        get_point(left_current),
                        get_point(right_current),
                        get_point(n),
                    ) > 0.0
                    {
                        valid_neighbors.push(n);
                    }
                }

                let base_ray: Vec2 = get_point(left_current) - get_point(right_current);
                valid_neighbors.sort_by(|&a, &b| {
                    let ray_a: Vec2 = get_point(a) - get_point(right_current);
                    let ray_b: Vec2 = get_point(b) - get_point(right_current);
                    let angle_a: f32 = base_ray.perp_dot(ray_a).atan2(base_ray.dot(ray_a));
                    let angle_b: f32 = base_ray.perp_dot(ray_b).atan2(base_ray.dot(ray_b));
                    angle_b.partial_cmp(&angle_a).unwrap()
                });

                let mut i: usize = 0;
                while i < valid_neighbors.len() {
                    if i + 1 < valid_neighbors.len() {
                        let c: TriangulationCircle = TriangulationCircle {
                            a: get_point(left_current),
                            b: get_point(right_current),
                            c: get_point(valid_neighbors[i]),
                        };
                        if c.in_circle(get_point(valid_neighbors[i + 1])) {
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

            let mut left_candidate: Option<usize> = None;
            if let Some(neighbors) = left_adjacency.get(&left_current) {
                let mut valid_neighbors: Vec<usize> = Vec::new();
                for &n in neighbors {
                    if !left_edges.contains(&(left_current, n))
                        && !left_edges.contains(&(n, left_current))
                    {
                        continue;
                    }
                    if counter_clockwise(
                        get_point(left_current),
                        get_point(right_current),
                        get_point(n),
                    ) > 0.0
                    {
                        valid_neighbors.push(n);
                    }
                }

                let base_ray: Vec2 = get_point(right_current) - get_point(left_current);
                valid_neighbors.sort_by(|&a, &b| {
                    let ray_a: Vec2 = get_point(a) - get_point(left_current);
                    let ray_b: Vec2 = get_point(b) - get_point(left_current);
                    let angle_a: f32 = base_ray.perp_dot(ray_a).atan2(base_ray.dot(ray_a));
                    let angle_b: f32 = base_ray.perp_dot(ray_b).atan2(base_ray.dot(ray_b));
                    angle_a.partial_cmp(&angle_b).unwrap()
                });

                let mut i: usize = 0;
                while i < valid_neighbors.len() {
                    if i + 1 < valid_neighbors.len() {
                        let c: TriangulationCircle = TriangulationCircle {
                            a: get_point(left_current),
                            b: get_point(right_current),
                            c: get_point(valid_neighbors[i]),
                        };
                        if c.in_circle(get_point(valid_neighbors[i + 1])) {
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

            let is_choosing_right: bool = match (left_candidate, right_candidate) {
                (None, None) => break,
                (Some(_), None) => false,
                (None, Some(_)) => true,
                (Some(lc), Some(rc)) => {
                    let circle: TriangulationCircle = TriangulationCircle {
                        a: get_point(left_current),
                        b: get_point(right_current),
                        c: get_point(lc),
                    };
                    circle.in_circle(get_point(rc))
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

    pub fn from_points(points: &[[f32; 2]]) -> Self {
        if points.len() < 3 {
            return Self::default();
        }

        let mut x_sorted_indices: Vec<usize> = (0..points.len()).collect();
        x_sorted_indices
            .sort_unstable_by(|&a, &b| points[a][0].partial_cmp(&points[b][0]).unwrap());

        let edges: HashSet<(usize, usize)> =
            TriangulationMesh::triangulation(points, &x_sorted_indices, 0, points.len() - 1);

        let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); points.len()];
        for &(u, v) in &edges {
            adjacency[u].push(v);
            adjacency[v].push(u);
        }

        let mut triangles: HashSet<[usize; 3]> = HashSet::new();

        for &(u, v) in &edges {
            for &w in &adjacency[u] {
                if adjacency[v].contains(&w) {
                    let p_u: Vec2 = vec2(points[u][0], points[u][1]);
                    let p_v: Vec2 = vec2(points[v][0], points[v][1]);
                    let p_w: Vec2 = vec2(points[w][0], points[w][1]);

                    let triangle: [usize; 3] = if (p_v - p_u).perp_dot(p_w - p_u) > 0.0 {
                        [u, v, w]
                    } else {
                        [u, w, v]
                    };

                    let min_index: usize = if triangle[0] < triangle[1] && triangle[0] < triangle[2]
                    {
                        0
                    } else if triangle[1] < triangle[0] && triangle[1] < triangle[2] {
                        1
                    } else {
                        2
                    };

                    let normalized_triangle: [usize; 3] = match min_index {
                        0 => [triangle[0], triangle[1], triangle[2]],
                        1 => [triangle[1], triangle[2], triangle[0]],
                        _ => [triangle[2], triangle[0], triangle[1]],
                    };

                    triangles.insert(normalized_triangle);
                }
            }
        }

        let triangles: Vec<[usize; 3]> = triangles.into_iter().collect();

        Self::from_triangles(points, &triangles)
    }

    pub fn from_triangles(points: &[[f32; 2]], triangles: &[[usize; 3]]) -> Self {
        let mut mesh: TriangulationMesh = TriangulationMesh {
            vertices: points
                .iter()
                .map(|&pos| Vertex {
                    pos,
                    incident_edge: None,
                })
                .collect(),
            half_edges: Vec::with_capacity(triangles.len() * 3),
            faces: Vec::with_capacity(triangles.len()),
        };

        let mut edge_map: HashMap<(usize, usize), HalfEdgeIndex> = HashMap::new();

        for (face_index, &triangle) in triangles.iter().enumerate() {
            let half_edge_start_index: usize = mesh.half_edges.len();

            mesh.faces.push(Face {
                edge: half_edge_start_index,
            });

            for i in 0..3 {
                let v_current: VertexIndex = triangle[i];
                let v_next: VertexIndex = triangle[(i + 1) % 3];

                let half_edge_index: HalfEdgeIndex = half_edge_start_index + i;
                let half_edge_next_index: HalfEdgeIndex = half_edge_start_index + ((i + 1) % 3);

                mesh.half_edges.push(HalfEdge {
                    origin: v_current,
                    twin: None,
                    next: half_edge_next_index,
                    face: face_index,
                });

                mesh.vertices[v_current].incident_edge = Some(half_edge_index);

                if let Some(&twin_index) = edge_map.get(&(v_next, v_current)) {
                    mesh.half_edges[half_edge_index].twin = Some(twin_index);
                    mesh.half_edges[twin_index].twin = Some(half_edge_index);
                } else {
                    edge_map.insert((v_current, v_next), half_edge_index);
                }
            }
        }

        mesh
    }

    pub fn flip_edge(&mut self, edge_index: HalfEdgeIndex) -> bool {
        let twin_index: HalfEdgeIndex = match self.half_edges[edge_index].twin {
            Some(index) => index,
            None => return false,
        };

        let edge_next_twin_index: HalfEdgeIndex = self.half_edges[edge_index].next;
        let edge_previous_index: HalfEdgeIndex = self.half_edges[edge_next_twin_index].next;

        let triangle_next_twin_index: HalfEdgeIndex = self.half_edges[twin_index].next;
        let t_prev_index: HalfEdgeIndex = self.half_edges[triangle_next_twin_index].next;

        let v1_index: VertexIndex = self.half_edges[edge_index].origin;
        let v2_index: VertexIndex = self.half_edges[twin_index].origin;
        let v3_index: VertexIndex = self.half_edges[edge_previous_index].origin;
        let v4_index: VertexIndex = self.half_edges[t_prev_index].origin;

        self.half_edges[edge_index].origin = v4_index;
        self.half_edges[twin_index].origin = v3_index;

        self.half_edges[edge_index].next = edge_previous_index;
        self.half_edges[edge_previous_index].next = triangle_next_twin_index;
        self.half_edges[triangle_next_twin_index].next = edge_index;

        self.half_edges[twin_index].next = t_prev_index;
        self.half_edges[t_prev_index].next = edge_next_twin_index;
        self.half_edges[edge_next_twin_index].next = twin_index;

        let face_1: FaceIndex = self.half_edges[edge_index].face;
        let face_2: FaceIndex = self.half_edges[twin_index].face;

        self.half_edges[triangle_next_twin_index].face = face_1;
        self.half_edges[edge_next_twin_index].face = face_2;

        self.faces[face_1].edge = edge_index;
        self.faces[face_2].edge = twin_index;

        self.vertices[v1_index].incident_edge = Some(edge_next_twin_index);
        self.vertices[v2_index].incident_edge = Some(triangle_next_twin_index);
        self.vertices[v3_index].incident_edge = Some(twin_index);
        self.vertices[v4_index].incident_edge = Some(edge_index);

        true
    }

    pub fn update_delaunay(&mut self) {
        let mut edges_to_check: Vec<HalfEdgeIndex> = (0..self.half_edges.len()).collect();
        let mut flipped_this_frame: HashSet<HalfEdgeIndex> = HashSet::new();

        while let Some(edge_index) = edges_to_check.pop() {
            if flipped_this_frame.contains(&edge_index)
                || self.half_edges[edge_index].twin.is_none()
            {
                continue;
            }

            let twin_index: HalfEdgeIndex = self.half_edges[edge_index].twin.unwrap();

            let v_a: Vec2 = self.vertices[self.half_edges[edge_index].origin].pos.into();
            let v_b: Vec2 = self.vertices[self.half_edges[twin_index].origin].pos.into();

            let edge_previous: HalfEdgeIndex =
                self.half_edges[self.half_edges[edge_index].next].next;
            let v_c: Vec2 = self.vertices[self.half_edges[edge_previous].origin]
                .pos
                .into();

            let t_prev: HalfEdgeIndex = self.half_edges[self.half_edges[twin_index].next].next;
            let v_d: Vec2 = self.vertices[self.half_edges[t_prev].origin].pos.into();

            if in_circle(v_c, v_a, v_b, v_d) {
                self.flip_edge(edge_index);

                flipped_this_frame.insert(edge_index);
                flipped_this_frame.insert(twin_index);

                edges_to_check.push(self.half_edges[edge_index].next);
                edges_to_check.push(self.half_edges[edge_previous].next);
                edges_to_check.push(self.half_edges[twin_index].next);
                edges_to_check.push(self.half_edges[t_prev].next);
            }
        }
    }
}
