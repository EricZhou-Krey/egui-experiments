use glam::Vec2;
use slotmap::basic::Values;

use crate::{
    scene::{SceneObjectKey, scene_object::{SceneObject, Shape}},
    settings::{
        style::{FaceStyle, LineStyle, PointStyle},
        style_sheet::{
            MAP_GRID_LINE_COLOR_MULTIPLIER, MAP_GRID_LINE_WIDTH, MAP_GRID_MIN_SCREEN_SPACING, MAP_GRID_SCALE_FACTOR,
            MAP_GRID_TEXT_COLOR_MULTIPLIER, MAP_GRID_TEXT_OFFSET_X, MAP_GRID_TEXT_OFFSET_Y_X_AXIS, MAP_GRID_TEXT_OFFSET_Y_Y_AXIS,
            MAP_GRID_TEXT_SIZE, MAP_TOOLBAR_BUTTON_SIZE, MAP_TOOLBAR_CORNER_RADIUS, MAP_TOOLBAR_MARGIN, MAP_TOOLBAR_PADDING
        }
    },
    state::{TTSState, map::{MapAction, MapTool}}};

pub fn mapview_title(_state: &mut TTSState) -> egui::WidgetText {
    "MapView".into()
}

pub fn mapview_ui(state: &mut TTSState, ui: &mut egui::Ui) {
    let dock_rectangle: egui::Rect = ui.available_rect_before_wrap();
    let toolbar_pos: egui::Pos2 = dock_rectangle.min + MAP_TOOLBAR_PADDING;
    let toolbar_position: Vec2 = Vec2::new(toolbar_pos.x, toolbar_pos.y);


    grid(state, ui);
    scene_view(state, ui);
    toolbar(state, ui, toolbar_position);
}

fn grid(state: &mut TTSState, ui: &mut egui::Ui) {
    let painter: &egui::Painter = ui.painter();
    
    let rect: egui::Rect = ui.available_rect_before_wrap();

    let min_screen: Vec2 = Vec2::new(rect.min.x, rect.min.y);
    let max_screen: Vec2 = Vec2::new(rect.max.x, rect.max.y);

    let min_world: Vec2 = state.map.screen_to_world(min_screen);
    let max_world: Vec2 = state.map.screen_to_world(max_screen);

    let mut grid_w: f32 = state.map.grid_cell_width;
    let mut grid_h: f32 = state.map.grid_cell_height;

    while grid_w * state.map.zoom < MAP_GRID_MIN_SCREEN_SPACING {
        grid_w *= MAP_GRID_SCALE_FACTOR;
    }
    while grid_h * state.map.zoom < MAP_GRID_MIN_SCREEN_SPACING {
        grid_h *= MAP_GRID_SCALE_FACTOR;
    }

    let start_x: isize = (min_world.x / grid_w).floor() as isize;
    let end_x: isize = (max_world.x / grid_w).ceil() as isize;

    let start_y: isize = (min_world.y / grid_h).floor() as isize;
    let end_y: isize = (max_world.y / grid_h).ceil() as isize;

    let line_color = ui.visuals().widgets.noninteractive.bg_stroke.color.linear_multiply(MAP_GRID_LINE_COLOR_MULTIPLIER);
    let stroke = egui::Stroke::new(MAP_GRID_LINE_WIDTH, line_color);
    let text_color = ui.visuals().text_color().linear_multiply(MAP_GRID_TEXT_COLOR_MULTIPLIER);
    let font_id = egui::FontId::proportional(MAP_GRID_TEXT_SIZE);

    for i in start_x..=end_x {
        let world_x: f32 = (i as f32) * grid_w;
        let screen_x: f32 = state.map.world_to_screen(Vec2::new(world_x, 0.0)).x;

        painter.line_segment(
            [
                egui::Pos2::new(screen_x, rect.min.y),
                egui::Pos2::new(screen_x, rect.max.y),
            ],
            stroke,
        );

        painter.text(
            egui::Pos2::new(screen_x + MAP_GRID_TEXT_OFFSET_X, rect.min.y + MAP_GRID_TEXT_OFFSET_Y_X_AXIS),
            egui::Align2::LEFT_TOP,
            format!("{:.1}", world_x),
            font_id.clone(),
            text_color,
        );
    }

    for i in start_y..=end_y {
        let world_y: f32 = (i as f32) * grid_h;
        let screen_y: f32 = state.map.world_to_screen(Vec2::new(0.0, world_y)).y;

        painter.line_segment(
            [
                egui::Pos2::new(rect.min.x, screen_y),
                egui::Pos2::new(rect.max.x, screen_y),
            ],
            stroke,
        );

        painter.text(
            egui::Pos2::new(rect.min.x + MAP_GRID_TEXT_OFFSET_X, screen_y + MAP_GRID_TEXT_OFFSET_Y_Y_AXIS),
            egui::Align2::LEFT_TOP,
            format!("{:.1}", world_y),
            font_id.clone(),
            text_color,
        );
    }
}

fn scene_view(state: &mut TTSState, ui: &mut egui::Ui) {
    MapTool::interact(state, ui);

    let painter: &egui::Painter = ui.painter();

    fn draw_vertex(zoom: f32, painter: &egui::Painter, screen_position: &Vec2, point_style: &PointStyle) {
        painter.add(egui::Shape::circle_filled(
            egui::Pos2::new(screen_position.x, screen_position.y),
            point_style.radius * zoom,
            point_style.color,
        ));
    }

    fn draw_line(zoom: f32, painter: &egui::Painter, screen_a: &Vec2, screen_b: &Vec2, line_style: &LineStyle) {
        painter.add(egui::Shape::line_segment(
            [egui::Pos2::new(screen_a.x, screen_a.y),
            egui::Pos2::new(screen_b.x, screen_b.y)],
            egui::Stroke::new(line_style.width * zoom, line_style.color),
        ));
    }

    fn draw_polygon(painter: &egui::Painter, screen_positions: &[Vec2], face_style: &FaceStyle) {
        painter.add(egui::Shape::convex_polygon(
            screen_positions.iter().map(|p| egui::Pos2::new(p.x, p.y)).collect(),
            face_style.fill_color,
            egui::Stroke::NONE,
        ));
    }

    let draw_shape = |shape: &Shape| {
        match shape {
            Shape::Point(position, point_style) => {
                let screen_position: Vec2 = state.map.world_to_screen(*position);
                draw_vertex(state.map.zoom, painter, &screen_position, point_style);
            }
            Shape::Line(a, b, line_style, point_style) => {
                let screen_a: Vec2 = state.map.world_to_screen(*a);
                let screen_b: Vec2 = state.map.world_to_screen(*b);

                draw_line(state.map.zoom, painter, &screen_a, &screen_b, line_style);
                if let Some(point_style) = point_style {
                    draw_vertex(state.map.zoom, painter, &screen_a, point_style);
                    draw_vertex(state.map.zoom, painter, &screen_b, point_style);
                }
            }
            Shape::Polygon(vertices, face_style, line_style, point_style) => {
                let screen_positions: Vec<Vec2> = vertices.iter().map(|point: &Vec2| {
                    state.map.world_to_screen(*point)
                }).collect();

                draw_polygon(painter, &screen_positions, face_style);

                if vertices.len() > 2 && let Some(line_style) = line_style {
                    for screen_line_vertices in screen_positions.windows(2) {
                        draw_line(state.map.zoom, painter, &screen_line_vertices[0], &screen_line_vertices[1], line_style);
                    }
                }

                if let Some(point_style) = point_style {
                    for screen_position in screen_positions {
                        draw_vertex(state.map.zoom, painter, &screen_position, point_style);
                    }
                }
            }
        }
    };


    let scene_objects: Values<'_, SceneObjectKey, SceneObject> = state.view_scene().objects();
    for object in scene_objects {
        draw_shape(object.shape());
    }

    if let MapAction::AddingPolygon(shape) = &state.map.action_in_progress {
        draw_shape(shape);
    }
}

fn toolbar(state: &mut TTSState, ui: &mut egui::Ui, position: Vec2) {
    egui::Area::new("MapToolbar".into())
        .fixed_pos(egui::Pos2::new(position.x, position.y))
        .interactable(true)
        .show(ui, |ui: &mut egui::Ui| {
            egui::Frame::window(ui.style())
                .inner_margin(MAP_TOOLBAR_MARGIN)
                .corner_radius(MAP_TOOLBAR_CORNER_RADIUS)
                .show(ui, |ui: &mut egui::Ui| {
                    let mut tool_selected: Vec<bool> = vec![false; MapTool::ALL.len()];
                    tool_selected[state.map.map_selected_tool.clone() as usize] = true;

                    for map_selected_tool in MapTool::ALL {
                        if ui
                            .add(
                                egui::Button::new(map_selected_tool.icon())
                                    .selected(tool_selected[map_selected_tool.clone() as usize])
                                    .min_size(MAP_TOOLBAR_BUTTON_SIZE),
                            )
                            .clicked()
                        {
                            state.map.action_in_progress = MapAction::None;
                            state.map.map_selected_tool = map_selected_tool.clone();
                        }
                    }
                })
        });
}
