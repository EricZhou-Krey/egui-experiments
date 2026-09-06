use std::ops::{Deref, DerefMut};

use crate::{
    settings::{
        style_sheet::TRIANGULATION_GRAPH_STYLE, InteractableTriangulationMeshSettings,
        TriangulationGraphSettings,
    },
    style::GraphStyle,
    triangulation::mesh::{AnimatedTriangulationMesh, HalfEdge},
};
use egui::{Painter, Pos2, Rect, Shape, Ui};
use glam::{vec2, Vec2};

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
                settings.n_internal_vertices,
                settings.vertex_speed,
            ),
            interactable_vertices: rand::seq::index::sample(
                &mut rand::rng(),
                settings.n_internal_vertices,
                settings.n_interactable,
            )
            .into_iter()
            .map(|i| i + 4)
            .collect(),
            interact_vertex: None,
            settings,
        }
    }

    fn interact(&mut self, position: Vec2) -> InteractionType {
        let mut new_interaction: Option<usize> = None;
        let mut interaction_radius: f32 = self.settings.interaction_radius;

        for v_index in self.interactable_vertices.iter() {
            let distance: f32 = position.distance(self.vertices[*v_index].pos);

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
pub struct GraphViewTransform {
    pub rect: Rect,
    pub render_scale: f32,
    pub render_offset_x: f32,
    pub render_offset_y: f32,
}

impl GraphViewTransform {
    pub fn new(rect: Rect, mesh_zoom: f32) -> Self {
        let base_scale = rect.width().max(rect.height()).max(1.0);
        let render_scale = base_scale * mesh_zoom;
        let render_offset_x = (rect.width() - render_scale) / 2.0;
        let render_offset_y = (rect.height() - render_scale) / 2.0;

        Self {
            rect,
            render_scale,
            render_offset_x,
            render_offset_y,
        }
    }

    pub fn to_screen(&self, position: Vec2) -> Pos2 {
        Pos2::new(
            self.rect.min.x + self.render_offset_x + (position[0] * self.render_scale),
            self.rect.min.y + self.render_offset_y + (position[1] * self.render_scale),
        )
    }

    pub fn to_uv(&self, position: Pos2) -> Vec2 {
        vec2(
            (position.x - self.rect.min.x - self.render_offset_x) / self.render_scale,
            (position.y - self.rect.min.y - self.render_offset_y) / self.render_scale,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TriangulationGraph {
    pub mesh: InteractableTriangulationMesh,
    pub graph_view_transform: GraphViewTransform,
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
            graph_view_transform: GraphViewTransform::new(Rect::ZERO, graph_settings.mesh_zoom),
            style: TRIANGULATION_GRAPH_STYLE,
            settings: graph_settings,
        }
    }
}

impl eframe::App for TriangulationGraph {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        let rect: Rect = ui.available_rect_before_wrap();

        ui.request_repaint();

        if self.graph_view_transform.rect != rect {
            self.graph_view_transform = GraphViewTransform::new(rect, self.settings.mesh_zoom);
        }

        let painter: Painter = ui.painter().with_clip_rect(rect);

        for face in self.mesh.faces.iter() {
            let mut points: Vec<Pos2> = Vec::new();
            let mut current_half_edge: &HalfEdge = &self.mesh.half_edges[face.edge];

            for _ in 0..3 {
                let raw_position: Vec2 = self.mesh.vertices[current_half_edge.origin].pos;
                points.push(self.graph_view_transform.to_screen(raw_position));
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
                let p1: Pos2 = self
                    .graph_view_transform
                    .to_screen(self.mesh.vertices[edge.origin].pos);
                let p2: Pos2 = self
                    .graph_view_transform
                    .to_screen(self.mesh.vertices[self.mesh.half_edges[edge.next].origin].pos);

                painter.line_segment([p1, p2], self.style.line.stroke);
            }
        }

        for v_index in self.mesh.interactable_vertices.iter() {
            let raw_position: Vec2 = self.mesh.vertices[*v_index].pos;
            let screen_position: Pos2 = self.graph_view_transform.to_screen(raw_position);

            painter.circle_filled(
                screen_position,
                self.style.point.radius,
                self.style.point.color,
            );
        }

        if let Some(interacted_index) = self.mesh.interact_vertex {
            painter.circle_filled(
                self.graph_view_transform
                    .to_screen(self.mesh.vertices[interacted_index].pos),
                self.style.point_heavy.radius,
                self.style.point_heavy.color,
            );
        }
    }

    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut mouse_click_position: Option<Pos2> = None;
        ctx.input(|i| {
            if i.pointer.primary_pressed() {
                mouse_click_position = i.pointer.interact_pos();
            }
        });

        if let Some(screen_position) = mouse_click_position {
            self.mesh
                .interact(self.graph_view_transform.to_uv(screen_position));
        }

        let dt: f32 = ctx.input(|i| i.stable_dt).min(0.1);

        let bounds: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.mesh.update(dt, bounds);
    }
}
