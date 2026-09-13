use egui::{Color32, Painter, Pos2, Rect, Stroke, Ui};
use glam::{vec2, Vec2};

use super::boid::Boid;
use crate::{
    graph::{GraphInteractNodeIndex, GraphUpdate}, settings::{BoidsGraphSettings, style_sheet::TRIANGULATION_GRAPH_STYLE},
};

#[derive(Debug, Clone, PartialEq)]
pub struct BoidsViewTransform {
    pub rect: Rect,
    pub render_scale: f32,
    pub render_offset_x: f32,
    pub render_offset_y: f32,
}

impl Default for BoidsViewTransform {
    fn default() -> Self {
        Self::new(Rect::ZERO, 1.0)
    }
}

impl BoidsViewTransform {
    pub fn new(rect: Rect, mesh_zoom: f32) -> Self {
        let base_scale: f32 = rect.width().max(rect.height()).max(1.0);
        let render_scale: f32 = base_scale * mesh_zoom;
        let render_offset_x: f32 = (rect.width() - render_scale) / 2.0;
        let render_offset_y: f32 = (rect.height() - render_scale) / 2.0;

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

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BoidGraph {
    pub boids: Vec<Boid>,
    pub settings: BoidsGraphSettings,
    pub graph_view_transform: BoidsViewTransform,
    pub interact_index: Option<GraphInteractNodeIndex>,
}

impl BoidGraph {
    pub fn new(settings: BoidsGraphSettings) -> Self {
        let mut boids: Vec<Boid> = Vec::with_capacity(settings.n_boids);
        for _ in 0..settings.n_boids {
            boids.push(Boid::new_random());
        }

        Self {
            boids,
            settings,
            graph_view_transform: BoidsViewTransform::new(Rect::ZERO, 1.0),
            interact_index: None,
        }
    }

    pub fn apply_settings(&mut self) {
        if self.boids.len() < self.settings.n_boids {
            let difference: usize = self.settings.n_boids - self.boids.len();
            for _ in 0..difference {
                self.boids.push(Boid::new_random());
            }
        } else if self.boids.len() > self.settings.n_boids {
            self.boids.truncate(self.settings.n_boids);
        }

        self.graph_view_transform =
            BoidsViewTransform::new(self.graph_view_transform.rect, self.settings.mesh_zoom);
    }

    pub fn clip_interactable_nodes(
        &mut self,
        blocked_screen_rects: Vec<Rect>,
        allowed_clip_rect: Rect,
    ) {
        let allowed_min: Vec2 = self.graph_view_transform.to_uv(allowed_clip_rect.min);
        let allowed_max: Vec2 = self.graph_view_transform.to_uv(allowed_clip_rect.max);
        let allowed_uv_rect: Rect = egui::Rect::from_min_max(
            egui::pos2(
                allowed_min.x.min(allowed_max.x),
                allowed_min.y.min(allowed_max.y),
            ),
            egui::pos2(
                allowed_min.x.max(allowed_max.x),
                allowed_min.y.max(allowed_max.y),
            ),
        );

        let blocked_uv_rects: Vec<Rect> = blocked_screen_rects
            .into_iter()
            .map(|screen_rect| {
                let min: Vec2 = self.graph_view_transform.to_uv(screen_rect.min);
                let max: Vec2 = self.graph_view_transform.to_uv(screen_rect.max);
                egui::Rect::from_min_max(
                    egui::pos2(min.x.min(max.x), min.y.min(max.y)),
                    egui::pos2(min.x.max(max.x), min.y.max(max.y)),
                )
            })
            .collect();

        // Only teleport the special interactable boids so they are always clickable
        for v_index in 0..self.settings.n_interactable {
            if v_index >= self.boids.len() {
                break;
            }

            let pos: Vec2 = self.boids[v_index].pos;
            let pos2: Pos2 = egui::pos2(pos.x, pos.y);

            let is_blocked: bool = blocked_uv_rects.iter().any(|r| r.contains(pos2));
            let is_outside: bool = !allowed_uv_rect.contains(pos2);

            if is_blocked || is_outside {
                for _ in 0..50 {
                    let rx: f32 =
                        allowed_uv_rect.min.x + rand::random::<f32>() * allowed_uv_rect.width();
                    let ry: f32 =
                        allowed_uv_rect.min.y + rand::random::<f32>() * allowed_uv_rect.height();
                    let candidate_pos: Pos2 = egui::pos2(rx, ry);

                    if !blocked_uv_rects.iter().any(|r| r.contains(candidate_pos)) {
                        self.boids[v_index].pos = glam::vec2(rx, ry);
                        break;
                    }
                }
            }
        }
    }

    pub fn logic(&mut self, ctx: &egui::Context) {
        let dt: f32 = ctx.input(|i| i.stable_dt).min(0.1);
        let mut forces: Vec<Vec2> = vec![Vec2::ZERO; self.boids.len()];

        for (i, force) in forces.iter_mut().enumerate() {
            let mut avg_pos: Vec2 = Vec2::ZERO;
            let mut avg_vel: Vec2 = Vec2::ZERO;
            let mut separation_force: Vec2 = Vec2::ZERO;
            let mut neighbors_count: usize = 0;

            let current_pos: Vec2 = self.boids[i].pos;
            let current_vel: Vec2 = self.boids[i].vel;

            for j in 0..self.boids.len() {
                if i == j {
                    continue;
                }

                let other_pos: Vec2 = self.boids[j].pos;
                let other_vel: Vec2 = self.boids[j].vel;

                let mut diff: Vec2 = current_pos - other_pos;
                if diff.x > 0.5 {
                    diff.x -= 1.0;
                } else if diff.x < -0.5 {
                    diff.x += 1.0;
                }
                if diff.y > 0.5 {
                    diff.y -= 1.0;
                } else if diff.y < -0.5 {
                    diff.y += 1.0;
                }

                let dist: f32 = diff.length();

                if dist < self.settings.perception_radius && dist > 0.0 {
                    avg_pos += other_pos;
                    avg_vel += other_vel;
                    neighbors_count += 1;

                    if dist < self.settings.separation_radius {
                        separation_force += diff.normalize_or_zero() / dist;
                    }
                }
            }

            if neighbors_count > 0 {
                let f_neighbors: f32 = neighbors_count as f32;
                avg_pos /= f_neighbors;
                avg_vel /= f_neighbors;

                let cohesion: Vec2 = (avg_pos - current_pos).normalize_or_zero()
                    * self.settings.max_speed
                    - current_vel;
                let alignment: Vec2 =
                    avg_vel.normalize_or_zero() * self.settings.max_speed - current_vel;

                *force = (cohesion * self.settings.cohesion_weight)
                    + (alignment * self.settings.alignment_weight)
                    + (separation_force * self.settings.separation_weight);
            }
        }

        for (i, boid) in self.boids.iter_mut().enumerate() {
            boid.vel += forces[i] * dt;

            let speed: f32 = boid.vel.length();
            if speed > self.settings.max_speed {
                boid.vel = (boid.vel / speed) * self.settings.max_speed;
            } else if speed < self.settings.min_speed && speed > 0.0 {
                boid.vel = (boid.vel / speed) * self.settings.min_speed;
            }

            boid.pos += boid.vel * dt;

            // UV Wrap bounds
            if boid.pos.x < 0.0 {
                boid.pos.x += 1.0;
            }
            if boid.pos.x > 1.0 {
                boid.pos.x -= 1.0;
            }
            if boid.pos.y < 0.0 {
                boid.pos.y += 1.0;
            }
            if boid.pos.y > 1.0 {
                boid.pos.y -= 1.0;
            }
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) -> Option<GraphUpdate> {
        let mut graph_update: Option<GraphUpdate> = None;
        let rect: Rect = ui.available_rect_before_wrap();
        let response = ui.allocate_rect(rect, egui::Sense::click());

        ui.request_repaint();

        if self.graph_view_transform.rect != rect {
            self.graph_view_transform = BoidsViewTransform::new(rect, self.settings.mesh_zoom);
        }

        // Only allow clicking on the first `n_interactable` boids
        if response.clicked()
            && let Some(screen_position) = response.interact_pointer_pos() {
                let uv_pos: Vec2 = self.graph_view_transform.to_uv(screen_position);

                let mut best_dist: f32 = self.settings.interact_radius;
                let mut best_index: Option<usize> = None;

                for i in 0..self.settings.n_interactable {
                    if i >= self.boids.len() {
                        break;
                    }
                    let dist: f32 = self.boids[i].pos.distance(uv_pos);
                    if dist < best_dist {
                        best_dist = dist;
                        best_index = Some(i);
                    }
                }

                if let Some(idx) = best_index {
                    let graph_idx = GraphInteractNodeIndex(idx);
                    if self.interact_index == Some(graph_idx) {
                        graph_update = Some(GraphUpdate::Reselect(graph_idx));
                    } else {
                        self.interact_index = Some(graph_idx);
                        graph_update = Some(GraphUpdate::Select(graph_idx));
                    }
                } else {
                    self.interact_index = None;
                    graph_update = Some(GraphUpdate::Deselect);
                }
            }

        let mut hovered_index: Option<usize> = None;
        if let Some(hover_pos) = response.hover_pos() {
            let uv_pos: Vec2 = self.graph_view_transform.to_uv(hover_pos);
            let mut best_dist: f32 = self.settings.interact_radius;

            for i in 0..self.settings.n_interactable {
                if i >= self.boids.len() {
                    break;
                }
                let dist: f32 = self.boids[i].pos.distance(uv_pos);
                if dist < best_dist {
                    best_dist = dist;
                    hovered_index = Some(i);
                }
            }
        }

        let painter: Painter = ui.painter().with_clip_rect(rect);
        
        let boid_color: Color32 = Color32::WHITE; 
        let point_color: Color32 = Color32::WHITE;         
        let heavy_color: Color32 = Color32::from_gray(180); 
        
        let boid_size: f32 = 6.0 * self.settings.mesh_zoom;
        
        // Pull exact sizes from Triangulation style
        let point_radius: f32 = TRIANGULATION_GRAPH_STYLE.point.radius;
        let point_heavy_radius: f32 = TRIANGULATION_GRAPH_STYLE.point_heavy.radius;

        for (i, boid) in self.boids.iter().enumerate() {
            let is_interactable: bool = i < self.settings.n_interactable;
            let is_selected: bool = self.interact_index.is_some_and(|idx| idx.0 == i);
            let screen_position: Pos2 = self.graph_view_transform.to_screen(boid.pos);
            
            let direction: Vec2 = boid.vel.normalize_or_zero();
            let perp: Vec2 = vec2(-direction.y, direction.x);

            let p1: Pos2 = screen_position + egui::vec2(direction.x, direction.y) * (boid_size * 1.5);
            let p2: Pos2 = screen_position - egui::vec2(direction.x, direction.y) * boid_size + egui::vec2(perp.x, perp.y) * boid_size;
            let p3: Pos2 = screen_position - egui::vec2(direction.x, direction.y) * boid_size - egui::vec2(perp.x, perp.y) * boid_size;

            // Draw the boid itself
            painter.add(egui::Shape::convex_polygon(
                vec![p1, p2, p3],
                boid_color, 
                Stroke::NONE,
            ));

            // Render Unselected Indicator (Exact Triangulation Match)
            if is_interactable && !is_selected {
                painter.circle_filled(screen_position, point_radius, point_color);
                
                painter.rect_stroke(
                    Rect::from_center_size(screen_position, egui::vec2(point_radius * 5.0, point_radius * 5.0)),
                    0.0,
                    Stroke::new(point_radius * 0.4, point_color),
                    egui::StrokeKind::Middle,
                );

                for delta in [egui::vec2(1., 0.), egui::vec2(-1., 0.), egui::vec2(0., 1.), egui::vec2(0., -1.)] {
                    painter.line_segment(
                        [
                            screen_position + (delta * point_radius * 1.5),
                            screen_position + (delta * point_radius * 3.5)
                        ],
                        Stroke::new(point_radius * 0.4, point_color)
                    );
                }
            }

            // Render Selected Heavy Indicator (Exact Triangulation Match)
            if is_selected {
                painter.circle_filled(screen_position, point_heavy_radius, heavy_color);

                painter.circle_stroke(
                    screen_position,
                    point_heavy_radius * 1.5,
                    Stroke::new(point_heavy_radius * 0.4, heavy_color),
                );
            }
        }

        if let Some(idx) = hovered_index && let Some(layout) = crate::layouts::Layout::ALL.get(idx) {
            let screen_pos = self.graph_view_transform.to_screen(self.boids[idx].pos);

            egui::Area::new(egui::Id::new("boid_tooltip").with(idx))
                .fixed_pos(screen_pos + egui::vec2(15.0, 15.0))
                .order(egui::Order::Tooltip) // Ensures it floats above other elements
                .show(ui.ctx(), |ui| {
                    crate::settings::style_sheet::LAYOUT_BLOCK_FRAME.show(ui, |ui| {
                        ui.strong(layout.name().to_uppercase());
                    });
                });
        }

        graph_update
    }
}
