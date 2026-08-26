use crate::{
    logic_sheet::{
        MAP_BASE_ZOOM, MAP_INTERACTION_RADIUS, MAP_ZOOM_LIMIT, MAP_ZOOM_SENSITIVITY,
        generate_sample_transmitter_sound},
        scene_object::{Emitter, Receiver, SceneObject, Shape, Wall},
        state::TTSState, style::{FaceStyle, LineStyle, MapStyle, PointStyle},
        style_sheet::{
        MAP_ADDEMITTER_ICON, MAP_ADDRECEIVER_ICON, MAP_ADDWALL_ICON, MAP_MOVE_ICON, MAP_PAN_ICON,
        MAP_REMOVE_ICON, MAP_SELECT_ICON, MAP_TOOLBAR_BUTTON_SIZE, MAP_TOOLBAR_CORNER_RADIUS,
        MAP_TOOLBAR_MARGIN, MAP_TOOLBAR_PADDING, MAP_ZOOM_ICON,
    }
};
use glam::Vec2;
use std::ops::{Deref, DerefMut};

#[derive(Default, Debug, Clone, PartialEq)]
pub enum MapTool {
    #[default]
    Pan,
    Select,
    Remove,
    AddReceiver,
    AddEmitter,
    AddWall,
    Move,
    Zoom,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum MapAction {
    #[default]
    None,
    AddingPolygon(Shape),
    Moving(usize),
}

impl MapTool {
    pub const ALL: &'static [MapTool] = &[
        MapTool::Pan,
        MapTool::Select,
        MapTool::Remove,
        MapTool::AddReceiver,
        MapTool::AddEmitter,
        MapTool::AddWall,
        MapTool::Move,
        MapTool::Zoom,
    ];

    pub fn interact(state: &mut TTSState, ui: &mut egui::Ui) {
        match state.map.map_selected_tool {
            MapTool::Zoom => ui.input(|input_state: &egui::InputState| {
                if input_state.pointer.primary_down() {
                    let pointer_delta: egui::Vec2 = input_state.pointer.delta();
                    state.map.zoom *= 1.0 + (pointer_delta.y * -MAP_ZOOM_SENSITIVITY);
                    if state.map.zoom < MAP_ZOOM_LIMIT {
                        state.map.zoom = MAP_ZOOM_LIMIT;
                    }
                }
            }),

            MapTool::Move => ui.input(|input_state: &egui::InputState| {
                if input_state.pointer.primary_pressed()
                    && let Some(pointer_position) = input_state.pointer.interact_pos()
                {
                    let screen_position: Vec2 = Vec2::new(pointer_position.x, pointer_position.y);
                    let world_position: Vec2 = state.map.screen_to_world(screen_position);
                    let search_radius: f32 = state.map.interaction_radius / state.map.zoom;
                    if let Some(object_index) = state
                        .scene
                        .find_object_index_around(world_position, search_radius)
                    {
                        state.map.action_in_progress = MapAction::Moving(object_index);
                    }
                }

                if input_state.pointer.primary_down()
                    && let MapAction::Moving(object_index) = state.map.action_in_progress
                {
                    let pointer_delta: egui::Vec2 = input_state.pointer.delta();
                    let delta_vec: Vec2 = Vec2::new(pointer_delta.x, pointer_delta.y);
                    let world_delta: Vec2 = delta_vec / state.map.zoom;

                    state.scene.objects[object_index].mut_shape().translate(world_delta);
                }

                if input_state.pointer.primary_released() {
                    state.map.action_in_progress = MapAction::None;
                }
            }),

            MapTool::Pan => ui.input(|input_state: &egui::InputState| {
                if input_state.pointer.primary_down() {
                    let pointer_delta: egui::Vec2 = input_state.pointer.delta();
                    let delta_vec: Vec2 = Vec2::new(pointer_delta.x, pointer_delta.y);
                    state.map.pan += delta_vec;
                }
            }),

            MapTool::Select => ui.input(|input_state: &egui::InputState| {
                if input_state.pointer.primary_clicked()
                    && let Some(pointer_position) = input_state.pointer.interact_pos()
                {
                    let screen_position: Vec2 = Vec2::new(pointer_position.x, pointer_position.y);
                    let world_position: Vec2 = state.map.screen_to_world(screen_position);
                    let search_radius: f32 = state.map.interaction_radius / state.map.zoom;
                    state.map.selected_object_index = state
                        .scene
                        .find_object_index_around(world_position, search_radius);
                }
            }),

            MapTool::Remove => ui.input(|input_state: &egui::InputState| {
                if input_state.pointer.primary_clicked()
                    && let Some(pointer_position) = input_state.pointer.interact_pos()
                {
                    let screen_position: Vec2 = Vec2::new(pointer_position.x, pointer_position.y);
                    let world_position: Vec2 = state.map.screen_to_world(screen_position);
                    let search_radius: f32 = state.map.interaction_radius / state.map.zoom;
                    if let Some(object_index) = state
                        .scene
                        .find_object_index_around(world_position, search_radius)
                    {
                        state.scene.objects.remove(object_index);
                    }
                }
            }),

            MapTool::AddReceiver => ui.input(|input_state: &egui::InputState| {
                if input_state.pointer.primary_clicked()
                    && let Some(pointer_position) = input_state.pointer.interact_pos()
                {
                    let screen_position: Vec2 = Vec2::new(pointer_position.x, pointer_position.y);
                    let world_position: Vec2 = state.map.screen_to_world(screen_position);

                    state.map.selected_object_index = Some(state.scene.objects.len());
                    state.scene.objects.push(SceneObject::Receiver(Box::new(Receiver {
                        shape: Shape::Point(world_position, state.map.style.receiver.clone()),
                    })));

                }
            }),

            MapTool::AddEmitter => ui.input(|input_state: &egui::InputState| {
                if input_state.pointer.primary_clicked()
                    && let Some(pointer_position) = input_state.pointer.interact_pos()
                {
                    let screen_position: Vec2 = Vec2::new(pointer_position.x, pointer_position.y);
                    let world_position: Vec2 = state.map.screen_to_world(screen_position);

                    state.map.selected_object_index = Some(state.scene.objects.len());
                    state.scene.objects.push(
                        SceneObject::Emitter(Box::new(Emitter {
                            shape: Shape::Point(world_position, state.map.style.transmitter.clone()),
                            sound_data: generate_sample_transmitter_sound(),
                        })));
                }
            }),

            MapTool::AddWall => ui.input(|input_state: &egui::InputState| {
                if input_state.pointer.primary_clicked()
                    && let Some(pointer_position) = input_state.pointer.interact_pos()
                {
                    let screen_position: Vec2 = Vec2::new(pointer_position.x, pointer_position.y);
                    let world_position: Vec2 = state.map.screen_to_world(screen_position);
                    match &mut state.map.action_in_progress {
                        MapAction::None => {
                            state.map.action_in_progress =
                                MapAction::AddingPolygon(
                                    Shape::Polygon(
                                        vec![world_position],
                                        state.map.style.wall_face.clone(),
                                        state.map.style.wall_line.clone(),
                                        state.map.style.wall_vertex.clone(),
                                    )
                                );
                        }
                        MapAction::AddingPolygon(shape) => {
                            if let Shape::Polygon(vertices, ..) = shape {
                                vertices.push(world_position);
                            }
                        }
                        MapAction::Moving(_) => {}
                    }
                }

                if input_state.focused && input_state.key_pressed(egui::Key::Enter) {
                    let completed_action: MapAction = std::mem::replace(
                        &mut state.map.action_in_progress,
                        MapAction::None,
                    );

                    if let MapAction::AddingPolygon(shape) = completed_action {

                        state.map.selected_object_index = Some(state.scene.objects.len());
                        state.scene.objects.push(SceneObject::Wall(Box::new(Wall {
                            shape}
                        )));
                    }
                }

                if input_state.focused && input_state.key_pressed(egui::Key::Escape) {
                    state.map.action_in_progress = MapAction::None;
                }
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapSettings {
    pub interaction_radius: f32,
    pub style: MapStyle,
}

impl Default for MapSettings {
    fn default() -> Self {
        Self {
            interaction_radius: MAP_INTERACTION_RADIUS,
            style: MapStyle::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapState {
    pub map_selected_tool: MapTool,
    pub action_in_progress: MapAction,
    pub selected_object_index: Option<usize>,
    pub settings: MapSettings,
    pub zoom: f32,
    pub pan: Vec2,
}

impl Default for MapState {
    fn default() -> Self {
        Self {
            map_selected_tool: MapTool::default(),
            action_in_progress: MapAction::default(),
            selected_object_index: None,
            settings: MapSettings::default(),
            zoom: MAP_BASE_ZOOM,
            pan: Vec2::ZERO,
        }
    }
}

impl MapState {
    pub fn world_to_screen(&self, world_position: Vec2) -> Vec2 {
        (world_position * self.zoom) + self.pan
    }

    pub fn screen_to_world(&self, screen_position: Vec2) -> Vec2 {
        (screen_position - self.pan) / self.zoom
    }
}

impl Deref for MapState {
    type Target = MapSettings;
    fn deref(&self) -> &Self::Target {
        &self.settings
    }
}

impl DerefMut for MapState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.settings
    }
}

impl MapTool {
    pub fn icon(&self) -> egui::WidgetText {
        match self {
            MapTool::Pan => MAP_PAN_ICON.into(),
            MapTool::Select => MAP_SELECT_ICON.into(),
            MapTool::Remove => MAP_REMOVE_ICON.into(),
            MapTool::AddReceiver => MAP_ADDRECEIVER_ICON.into(),
            MapTool::AddEmitter => MAP_ADDEMITTER_ICON.into(),
            MapTool::AddWall => MAP_ADDWALL_ICON.into(),
            MapTool::Move => MAP_MOVE_ICON.into(),
            MapTool::Zoom => MAP_ZOOM_ICON.into(),
        }
    }
}

pub fn mapview_title(_state: &mut TTSState) -> egui::WidgetText {
    "MapView".into()
}

pub fn mapview_ui(state: &mut TTSState, ui: &mut egui::Ui) {
    let dock_rectangle: egui::Rect = ui.available_rect_before_wrap();
    let toolbar_pos: egui::Pos2 = dock_rectangle.min + MAP_TOOLBAR_PADDING;
    let toolbar_position: Vec2 = Vec2::new(toolbar_pos.x, toolbar_pos.y);

    main_view(state, ui);
    toolbar(state, ui, toolbar_position);
}

fn main_view(state: &mut TTSState, ui: &mut egui::Ui) {
    MapTool::interact(state, ui);

    let painter: &egui::Painter = ui.painter();
    
    let mut scene_shapes: Vec<&Shape> = state.scene.objects.iter().map(|object| object.shape()).collect();
    if let MapAction::AddingPolygon(shape) = &state.map.action_in_progress {
        scene_shapes.push(shape);
    }

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

    for shape in scene_shapes {
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
