use std::ops::{Deref, DerefMut};

use crate::{
    style::{FaceStyle, GraphStyle, LineStyle, PointStyle},
    triangulation_mesh::{AnimatedTriangulationMesh, HalfEdge},
};
use egui::{Color32, Painter, Pos2, Rect, Shape, Stroke, Ui};
use shared_view::viewable::Viewable;

#[derive(Debug, Clone, PartialEq)]
pub struct InteractableTriangulationMeshSettings {
    pub n_internal_verticies: usize,
    pub vertex_speed: f32,
    pub n_interactable: usize,
    pub interaction_radius: f32,
}

impl Default for InteractableTriangulationMeshSettings {
    fn default() -> Self {
        Self {
            n_internal_verticies: 200,
            vertex_speed: 0.01,
            n_interactable: 10,
            interaction_radius: 0.05,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct InteractableTriangulationMesh {
    pub animated_mesh: AnimatedTriangulationMesh,
    pub interactable_vertices: Vec<usize>,
    pub interact_vertex: Option<usize>,
    pub settings: InteractableTriangulationMeshSettings,
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

pub enum InteractionType {
    Deselect,
    Select(usize),
    Reselect(usize),
}

impl InteractableTriangulationMesh {
    fn new(settings: InteractableTriangulationMeshSettings) -> Self {
        InteractableTriangulationMesh {
            animated_mesh: AnimatedTriangulationMesh::new(
                settings.n_internal_verticies,
                settings.vertex_speed,
            ),
            interactable_vertices: rand::seq::index::sample(
                &mut rand::rng(),
                settings.n_internal_verticies,
                settings.n_interactable,
            )
            .into_iter()
            .map(|i| i + 4)
            .collect(),
            interact_vertex: None,
            settings,
        }
    }

    fn interact(&mut self, position: [f32; 2]) -> InteractionType {
        let mut new_interaction: Option<usize> = None;
        let mut interaction_radius: f32 = self.settings.interaction_radius;

        for v_index in self.interactable_vertices.iter() {
            let distance: f32 =
                glam::Vec2::from(position).distance(glam::Vec2::from(self.vertices[*v_index].pos));

            if distance < interaction_radius {
                interaction_radius = distance;
                new_interaction = Some(*v_index);
            }
        }

        match new_interaction {
            Some(index) => {
                if self.interact_vertex == Some(index) {
                    InteractionType::Reselect(index)
                } else {
                    self.interact_vertex = Some(index);
                    InteractionType::Select(index)
                }
            }
            None => {
                self.interact_vertex = None;
                InteractionType::Deselect
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TriangulationGraphSettings {
    pub mesh_zoom: f32,
}

impl Default for TriangulationGraphSettings {
    fn default() -> Self {
        Self { mesh_zoom: 1.15 }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TriangulationGraph {
    pub mesh: InteractableTriangulationMesh,
    pub style: GraphStyle,
    pub settings: TriangulationGraphSettings,
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
            style: GraphStyle {
                point: PointStyle {
                    radius: 5.0,
                    color: Color32::LIGHT_GREEN,
                },
                point_heavy: PointStyle {
                    radius: 7.0,
                    color: Color32::GREEN,
                },
                line: LineStyle {
                    stroke: Stroke::new(1.5, Color32::LIGHT_GRAY),
                },
                face: FaceStyle {
                    fill_color: Color32::from_rgb(40, 44, 52),
                    border_stroke: Stroke::NONE,
                },
                ..Default::default()
            },
            settings: graph_settings,
        }
    }
}

impl Viewable for TriangulationGraph {
    fn draw_ui(&mut self, ui: &mut Ui) {
        let rect: Rect = ui.available_rect_before_wrap();

        let base_scale: f32 = rect.width().max(rect.height());
        let base_offset_x: f32 = (rect.width() - base_scale) / 2.0;
        let base_offset_y: f32 = (rect.height() - base_scale) / 2.0;

        let render_scale: f32 = base_scale * self.settings.mesh_zoom;
        let render_offset_x: f32 = (rect.width() - render_scale) / 2.0;
        let render_offset_y: f32 = (rect.height() - render_scale) / 2.0;

        let to_screen = |position: [f32; 2]| -> Pos2 {
            Pos2::new(
                rect.min.x + render_offset_x + (position[0] * render_scale),
                rect.min.y + render_offset_y + (position[1] * render_scale),
            )
        };

        let to_uv = |position: Pos2| -> [f32; 2] {
            [
                (position.x - rect.min.x - render_offset_x) / render_scale,
                (position.y - rect.min.y - render_offset_y) / render_scale,
            ]
        };

        let mut mouse_click_position: Option<Pos2> = None;
        ui.input(|i| {
            if i.pointer.primary_pressed() {
                mouse_click_position = i.pointer.interact_pos();
            }
        });

        if let Some(screen_position) = mouse_click_position {
            self.mesh.interact(to_uv(screen_position));
        }

        let dt: f32 = ui.input(|i| i.stable_dt).min(0.1);

        let bounds: [f32; 4] = [
            -base_offset_x / base_scale,
            (rect.width() - base_offset_x) / base_scale,
            -base_offset_y / base_scale,
            (rect.height() - base_offset_y) / base_scale,
        ];

        self.mesh.update(dt, bounds);

        ui.request_repaint();

        let painter: Painter = ui.painter().with_clip_rect(rect);

        for face in self.mesh.faces.iter() {
            let mut points: Vec<Pos2> = Vec::new();
            let mut current_half_edge: &HalfEdge = &self.mesh.half_edges[face.edge];

            for _ in 0..3 {
                let raw_position: [f32; 2] = self.mesh.vertices[current_half_edge.origin].pos;
                points.push(to_screen(raw_position));
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
                let p1: Pos2 = to_screen(self.mesh.vertices[edge.origin].pos);
                let p2: Pos2 =
                    to_screen(self.mesh.vertices[self.mesh.half_edges[edge.next].origin].pos);

                painter.line_segment([p1, p2], self.style.line.stroke);
            }
        }

        for v_index in self.mesh.interactable_vertices.iter() {
            let raw_position: [f32; 2] = self.mesh.vertices[*v_index].pos;
            let screen_position: Pos2 = to_screen(raw_position);

            painter.circle_filled(
                screen_position,
                self.style.point.radius,
                self.style.point.color,
            );
        }

        if let Some(interacted_index) = self.mesh.interact_vertex {
            painter.circle_filled(
                to_screen(self.mesh.vertices[interacted_index].pos),
                self.style.point_heavy.radius,
                self.style.point_heavy.color,
            );
        }
    }
}
