use egui::{Color32, Painter, Pos2, Rect, Stroke, Ui};
use glam::{vec2, Vec2};

use super::board::Board;
use crate::{
    graph::{GraphInteractNodeIndex, GraphUpdate}, settings::{LifeGraphSettings, style_sheet::TRIANGULATION_GRAPH_STYLE},
};

#[derive(Debug, Clone, PartialEq)]
pub struct LifeViewTransform {
    pub rect: Rect,
    pub render_scale: f32,
    pub render_offset_x: f32,
    pub render_offset_y: f32,
}

impl LifeViewTransform {
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

    pub fn to_screen(&self, uv_position: Vec2) -> Pos2 {
        Pos2::new(
            self.rect.min.x + self.render_offset_x + (uv_position[0] * self.render_scale),
            self.rect.min.y + self.render_offset_y + (uv_position[1] * self.render_scale),
        )
    }

    pub fn to_uv(&self, screen_position: Pos2) -> Vec2 {
        vec2(
            (screen_position.x - self.rect.min.x - self.render_offset_x) / self.render_scale,
            (screen_position.y - self.rect.min.y - self.render_offset_y) / self.render_scale,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LifeGraph {
    pub board: Board,
    pub interactable_nodes: Vec<Vec2>, // Floating points in UV space [0.0, 1.0]
    pub settings: LifeGraphSettings,
    pub graph_view_transform: LifeViewTransform,
    pub interact_index: Option<GraphInteractNodeIndex>,
    pub time_since_last_tick: f32,
}

impl LifeGraph {
    pub fn new(settings: LifeGraphSettings) -> Self {
        let board: Board = Board::new(settings.cols, settings.rows, settings.initial_density);

        let mut interactable_nodes: Vec<Vec2> = Vec::with_capacity(settings.n_interactable);
        for _ in 0..settings.n_interactable {
            interactable_nodes.push(glam::vec2(rand::random::<f32>(), rand::random::<f32>()));
        }

        Self {
            board,
            interactable_nodes,
            settings,
            graph_view_transform: LifeViewTransform::new(Rect::ZERO, 1.0),
            interact_index: None,
            time_since_last_tick: 0.0,
        }
    }

    pub fn apply_settings(&mut self, reset_board: bool) {
        self.graph_view_transform =
            LifeViewTransform::new(self.graph_view_transform.rect, self.settings.mesh_zoom);

        if reset_board
            || self.board.cols != self.settings.cols
            || self.board.rows != self.settings.rows
        {
            self.board = Board::new(
                self.settings.cols,
                self.settings.rows,
                self.settings.initial_density,
            );
        }
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

        // Teleport floating nodes if they are hidden by a UI overlay
        for pos in self.interactable_nodes.iter_mut() {
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
                        *pos = glam::vec2(rx, ry);
                        break;
                    }
                }
            }
        }
    }

    pub fn logic(&mut self, ctx: &egui::Context) {
        let dt: f32 = ctx.input(|i| i.stable_dt).min(0.1);
        self.time_since_last_tick += dt;

        if self.time_since_last_tick >= self.settings.tick_rate {
            self.time_since_last_tick = 0.0;
            self.board.step();

            // Floating interactable nodes act as "spawners" and give life to the cell directly beneath them
            for node in &self.interactable_nodes {
                let cell_x: usize =
                    ((node.x * self.board.cols as f32) as usize).min(self.board.cols - 1);
                let cell_y: usize =
                    ((node.y * self.board.rows as f32) as usize).min(self.board.rows - 1);
                self.board.cells[cell_y * self.board.cols + cell_x] = true;
            }
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) -> Option<GraphUpdate> {
        let mut graph_update: Option<GraphUpdate> = None;
        let rect: Rect = ui.available_rect_before_wrap();
        let response = ui.allocate_rect(rect, egui::Sense::click());

        ui.request_repaint();

        if self.graph_view_transform.rect != rect {
            self.graph_view_transform = LifeViewTransform::new(rect, self.settings.mesh_zoom);
        }

        if response.clicked()
            && let Some(screen_position) = response.interact_pointer_pos() {
                let uv_pos: Vec2 = self.graph_view_transform.to_uv(screen_position);

                let mut best_dist: f32 = self.settings.interact_radius;
                let mut best_index: Option<usize> = None;

                for (i, node) in self.interactable_nodes.iter().enumerate() {
                    let dist: f32 = node.distance(uv_pos);
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

            for (i, node) in self.interactable_nodes.iter().enumerate() {
                let dist: f32 = node.distance(uv_pos);
                if dist < best_dist {
                    best_dist = dist;
                    hovered_index = Some(i);
                }
            }
        }

        let painter: Painter = ui.painter().with_clip_rect(rect);

        // Grayscale Palette
        let cell_color: Color32 = Color32::from_gray(180);
        let point_color: Color32 = Color32::WHITE;
        let heavy_color: Color32 = Color32::from_gray(180);

        // 1. Draw Grid Cells
        let cell_w: f32 = 1.0 / (self.board.cols as f32);
        let cell_h: f32 = 1.0 / (self.board.rows as f32);

        for y in 0..self.board.rows {
            for x in 0..self.board.cols {
                if self.board.cells[y * self.board.cols + x] {
                    let uv_min: Vec2 = glam::vec2(x as f32 * cell_w, y as f32 * cell_h);
                    let uv_max: Vec2 = glam::vec2((x + 1) as f32 * cell_w, (y + 1) as f32 * cell_h);

                    let p_min: Pos2 = self.graph_view_transform.to_screen(uv_min);
                    let p_max: Pos2 = self.graph_view_transform.to_screen(uv_max);

                    painter.rect_filled(Rect::from_min_max(p_min, p_max), 0.0, cell_color);
                }
            }
        }

        // 2. Draw Floating Indicators
        let point_radius: f32 = TRIANGULATION_GRAPH_STYLE.point.radius;
        let point_heavy_radius: f32 = TRIANGULATION_GRAPH_STYLE.point_heavy.radius;

        for (i, node) in self.interactable_nodes.iter().enumerate() {
            let is_selected: bool = self.interact_index.is_some_and(|idx| idx.0 == i);
            let screen_position: Pos2 = self.graph_view_transform.to_screen(*node);
            
            // Render Unselected Indicator (Exact Triangulation Match)
            if !is_selected {
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
            } else {
                // Render Selected Heavy Indicator (Exact Triangulation Match)
                painter.circle_filled(screen_position, point_heavy_radius, heavy_color);

                painter.circle_stroke(
                    screen_position,
                    point_heavy_radius * 1.5,
                    Stroke::new(point_heavy_radius * 0.4, heavy_color),
                );
            }
        }

        if let Some(idx) = hovered_index && let Some(layout) = crate::layouts::Layout::ALL.get(idx) {
            let screen_pos = self.graph_view_transform.to_screen(self.interactable_nodes[idx]);

            egui::Area::new(egui::Id::new("life_tooltip").with(idx))
                .fixed_pos(screen_pos + egui::vec2(15.0, 15.0))
                .order(egui::Order::Tooltip)
                .show(ui.ctx(), |ui| {
                    crate::settings::style_sheet::LAYOUT_BLOCK_FRAME.show(ui, |ui| {
                        ui.strong(layout.name().to_uppercase());
                    });
                });
        }

        graph_update
    }
}
