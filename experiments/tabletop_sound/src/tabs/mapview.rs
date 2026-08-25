use crate::{
    logic_sheet::{
        MAP_BASE_ZOOM, MAP_INTERACTION_RADIUS, MAP_ZOOM_LIMIT, MAP_ZOOM_SENSITIVITY, generate_sample_transmitter_sound
    }, scene_object::{Emitter, Receiver, SceneObject, Shape, Wall}, state::TTSState, style::MapStyle, style_sheet::{
        MAP_ADDEMITTER_ICON, MAP_ADDRECEIVER_ICON, MAP_ADDWALL_ICON, MAP_MOVE_ICON, MAP_PAN_ICON, MAP_REMOVE_ICON, MAP_SELECT_ICON, MAP_TOOLBAR_BUTTON_SIZE, MAP_TOOLBAR_CORNER_RADIUS, MAP_TOOLBAR_MARGIN, MAP_TOOLBAR_PADDING, MAP_ZOOM_ICON
    }
};
use egui::Painter;
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
    AddingPolygon(Vec<Vec2>),
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
                    state.scene.objects.push(
                        SceneObject::Emitter(Box::new(Emitter {
                            shape: Shape::Point(world_position, state.map.style.transmitter.clone()),
                            sound_data: generate_sample_transmitter_sound(),
                        })));

                    // TODO: TEMP TEST
                    state.scene.audio_manager.play(generate_sample_transmitter_sound()).unwrap();
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
                                MapAction::AddingPolygon(vec![world_position]);
                        }
                        MapAction::AddingPolygon(vertices) => {
                            vertices.push(world_position);
                        }
                        MapAction::Moving(_) => {}
                    }
                }

                if input_state.focused && input_state.key_pressed(egui::Key::Enter) {
                    let completed_action: MapAction = std::mem::replace(
                        &mut state.map.action_in_progress,
                        MapAction::None,
                    );

                    if let MapAction::AddingPolygon(vertices) = completed_action
                        && vertices.len() >= 3
                    {
                        state.scene.objects.push(SceneObject::Wall(Box::new(Wall {
                            shape: Shape::Polygon(vertices, state.map.style.wall.clone()),
                        })));
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

    let painter: &Painter = ui.painter();

    for scene_object in &state.scene.objects {
        match scene_object {
            SceneObject::Wall(wall) => {
                if let Shape::Polygon(vertices, face_style) = &wall.shape {
                    let points: Vec<egui::Pos2> = vertices
                        .iter()
                        .map(|point: &Vec2| {
                            let screen_position: Vec2 = state.map.world_to_screen(*point);
                            egui::Pos2::new(screen_position.x, screen_position.y)
                        })
                        .collect();

                    painter.add(egui::Shape::convex_polygon(
                        points,
                        face_style.fill_color,
                        face_style.border_stroke,
                    ));
                }
            }
            SceneObject::Receiver(receiver) => {
                if let Shape::Point(position, point_style) = &receiver.shape {
                    let screen_position: Vec2 = state.map.world_to_screen(*position);
                    painter.add(egui::Shape::circle_filled(
                        egui::Pos2::new(screen_position.x, screen_position.y),
                        point_style.radius * state.map.zoom,
                        point_style.color,
                    ));
                }
            }
            SceneObject::Emitter(emitter) => {
                if let Shape::Point(position, point_style) = &emitter.shape {
                    let screen_position: Vec2 = state.map.world_to_screen(*position);
                    painter.add(egui::Shape::circle_filled(
                        egui::Pos2::new(screen_position.x, screen_position.y),
                        point_style.radius * state.map.zoom,
                        point_style.color,
                    ));
                }
            }
        }
    }

    match &state.map.action_in_progress {
        MapAction::None => {}
        MapAction::AddingPolygon(vertices) => {
            for wall_vertex in vertices {
                let screen_position: Vec2 = state.map.world_to_screen(*wall_vertex);
                painter.add(egui::Shape::circle_filled(
                    egui::Pos2::new(screen_position.x, screen_position.y),
                    state.map.style.wall.vertex_radius * state.map.zoom,
                    state.map.style.wall.vertex_color,
                ));
            }
        }
        MapAction::Moving(_) => {}
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
