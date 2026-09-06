use glam::{vec2, Mat3, Vec2};
use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
};

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

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Vertex {
    pub pos: Vec2,
    pub incident_edge: Option<HalfEdgeIndex>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HalfEdge {
    pub origin: VertexIndex,
    pub twin: Option<HalfEdgeIndex>,
    pub next: HalfEdgeIndex,
    pub face: FaceIndex,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    pub edge: HalfEdgeIndex,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TriangulationMesh {
    pub vertices: Vec<Vertex>,
    pub half_edges: Vec<HalfEdge>,
    pub faces: Vec<Face>,
}

impl TriangulationMesh {
    pub fn from_points(points: &[Vec2]) -> Self {
        let n: usize = points.len();
        if n < 3 {
            return Self::default();
        }

        let mut x_sorted_indices: Vec<usize> = (0..n).collect();
        x_sorted_indices.sort_unstable_by(|&a, &b| points[a].x.partial_cmp(&points[b].x).unwrap());

        let edges: HashSet<(usize, usize)> =
            TriangulationMesh::triangulation(points, &x_sorted_indices, 0, n - 1);

        let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); n];
        for &(u, v) in &edges {
            adjacency[u].push(v);
            adjacency[v].push(u);
        }

        let mut triangles: HashSet<[usize; 3]> = HashSet::new();

        for &(u, v) in &edges {
            for &w in &adjacency[u] {
                if adjacency[v].contains(&w) {
                    let triangle: [usize; 3] =
                        if (points[v] - points[u]).perp_dot(points[w] - points[u]) > 0.0 {
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

    pub fn from_triangles(points: &[Vec2], triangles: &[[usize; 3]]) -> Self {
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

    fn triangulation(
        points: &[Vec2],
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

            if (points[b] - points[a])
                .perp_dot(points[c] - points[a])
                .abs()
                > 1e-4
            {
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
        points: &[Vec2],
        x_sorted_indicies: &[usize],
        mut left_edges: HashSet<(usize, usize)>,
        mut right_edges: HashSet<(usize, usize)>,
        left: usize,
        middle: usize,
        right: usize,
    ) -> HashSet<(usize, usize)> {
        let mut middle_edges: HashSet<(usize, usize)> = HashSet::new();
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
                if counter_clockwise(points[left_current], points[right_current], points[n]) < 0.0 {
                    left_current = n;
                    changed = true;
                }
            }

            for &n in &x_sorted_indicies[middle + 1..=right] {
                if n == right_current {
                    continue;
                }
                if counter_clockwise(points[left_current], points[right_current], points[n]) < 0.0 {
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
                    if counter_clockwise(points[left_current], points[right_current], points[n])
                        > 0.0
                    {
                        valid_neighbors.push(n);
                    }
                }

                let base_ray: Vec2 = points[left_current] - points[right_current];
                valid_neighbors.sort_by(|&a, &b| {
                    let ray_a: Vec2 = points[a] - points[right_current];
                    let ray_b: Vec2 = points[b] - points[right_current];
                    let angle_a: f32 = base_ray.perp_dot(ray_a).atan2(base_ray.dot(ray_a));
                    let angle_b: f32 = base_ray.perp_dot(ray_b).atan2(base_ray.dot(ray_b));
                    angle_b.partial_cmp(&angle_a).unwrap()
                });

                let mut i: usize = 0;
                while i < valid_neighbors.len() {
                    if i + 1 < valid_neighbors.len()
                        && in_circle(
                            points[left_current],
                            points[right_current],
                            points[valid_neighbors[i]],
                            points[valid_neighbors[i + 1]],
                        )
                    {
                        right_edges.remove(&(right_current, valid_neighbors[i]));
                        right_edges.remove(&(valid_neighbors[i], right_current));
                        i += 1;
                        continue;
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
                    if counter_clockwise(points[left_current], points[right_current], points[n])
                        > 0.0
                    {
                        valid_neighbors.push(n);
                    }
                }

                let base_ray: Vec2 = points[right_current] - points[left_current];
                valid_neighbors.sort_by(|&a, &b| {
                    let ray_a: Vec2 = points[a] - points[left_current];
                    let ray_b: Vec2 = points[b] - points[left_current];
                    let angle_a: f32 = base_ray.perp_dot(ray_a).atan2(base_ray.dot(ray_a));
                    let angle_b: f32 = base_ray.perp_dot(ray_b).atan2(base_ray.dot(ray_b));
                    angle_a.partial_cmp(&angle_b).unwrap()
                });

                let mut i: usize = 0;
                while i < valid_neighbors.len() {
                    if i + 1 < valid_neighbors.len()
                        && in_circle(
                            points[left_current],
                            points[right_current],
                            points[valid_neighbors[i]],
                            points[valid_neighbors[i + 1]],
                        )
                    {
                        left_edges.remove(&(left_current, valid_neighbors[i]));
                        left_edges.remove(&(valid_neighbors[i], left_current));
                        i += 1;
                        continue;
                    }

                    left_candidate = Some(valid_neighbors[i]);
                    break;
                }
            }

            let is_choosing_right: bool = match (left_candidate, right_candidate) {
                (None, None) => break,
                (Some(_), None) => false,
                (None, Some(_)) => true,
                (Some(lc), Some(rc)) => in_circle(
                    points[left_current],
                    points[right_current],
                    points[lc],
                    points[rc],
                ),
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
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimatedTriangulationMesh {
    pub velocities: Vec<Vec2>,
    pub mesh: TriangulationMesh,
}

impl Deref for AnimatedTriangulationMesh {
    type Target = TriangulationMesh;

    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}

impl DerefMut for AnimatedTriangulationMesh {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mesh
    }
}

impl AnimatedTriangulationMesh {
    pub fn new(n_internal_vertices: usize, speed: f32) -> Self {
        let mut points: Vec<Vec2> = vec![
            vec2(0.0, 0.0),
            vec2(1.0, 0.0),
            vec2(1.0, 1.0),
            vec2(0.0, 1.0),
        ];

        let mut velocities: Vec<Vec2> = vec![
            vec2(0.0, 0.0),
            vec2(0.0, 0.0),
            vec2(0.0, 0.0),
            vec2(0.0, 0.0),
        ];

        let random_points: Vec<Vec2> =
            std::iter::repeat_with(|| vec2(rand::random::<f32>(), rand::random::<f32>()))
                .take(n_internal_vertices)
                .collect();

        let random_velocities: Vec<Vec2> = std::iter::repeat_with(|| {
            let angle: f32 = rand::random::<f32>() * std::f32::consts::TAU;
            vec2(angle.cos(), angle.sin()) * speed
        })
        .take(n_internal_vertices)
        .collect();

        points.extend(random_points);
        velocities.extend(random_velocities);

        Self {
            velocities,
            mesh: TriangulationMesh::from_points(&points),
        }
    }

    pub fn update(&mut self, dt: f32, bounds: [f32; 4]) {
        let mut current_vertices: Vec<Vec2> =
            self.mesh.vertices.iter().map(|v: &Vertex| v.pos).collect();

        current_vertices[0] = vec2(bounds[0], bounds[2]);
        current_vertices[1] = vec2(bounds[1], bounds[2]);
        current_vertices[2] = vec2(bounds[1], bounds[3]);
        current_vertices[3] = vec2(bounds[0], bounds[3]);

        for (i, v) in self.velocities.iter_mut().enumerate() {
            if v.x == 0.0 && v.y == 0.0 {
                continue;
            }

            let mut pos: Vec2 = current_vertices[i];

            pos[0] += v.x * dt;
            pos[1] += v.y * dt;

            if pos[0] < bounds[0] {
                pos[0] = 2.0 * bounds[0] - pos[0];
                pos[0] = pos[0].max(bounds[0]).min(bounds[1]);
                v.x = v.x.abs();
            } else if pos[0] > bounds[1] {
                pos[0] = 2.0 * bounds[1] - pos[0];
                pos[0] = pos[0].max(bounds[0]).min(bounds[1]);
                v.x = -v.x.abs();
            }

            if pos[1] < bounds[2] {
                pos[1] = 2.0 * bounds[2] - pos[1];
                pos[1] = pos[1].max(bounds[2]).min(bounds[3]);
                v.y = v.y.abs();
            } else if pos[1] > bounds[3] {
                pos[1] = 2.0 * bounds[3] - pos[1];
                pos[1] = pos[1].max(bounds[2]).min(bounds[3]);
                v.y = -v.y.abs();
            }

            current_vertices[i] = pos;
        }

        self.mesh = TriangulationMesh::from_points(&current_vertices);
    }
}
