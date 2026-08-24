use crate::{
    logic_sheet::{MAP_INTERACTION_RADIUS, generate_sample_transmitter_sound},
    scene::{Receiver, SceneObject, Transmitter, Wall},
    state::TTSState,
    style::MapStyle,
    style_sheet::{
        MAP_ADDRECEIVER_ICON, MAP_ADDTRANSMITTER_ICON, MAP_ADDWALL_ICON, MAP_MOVE_ICON,
        MAP_PAN_ICON, MAP_REMOVE_ICON, MAP_SELECT_ICON, MAP_TOOLBAR_BUTTON_SIZE,
        MAP_TOOLBAR_CORNER_RADIUS, MAP_TOOLBAR_MARGIN, MAP_TOOLBAR_PADDING, MAP_ZOOM_ICON,
    },
};
use egui::{Painter, Shape};
use std::ops::{Deref, DerefMut};

#[derive(Default, Debug, Clone, PartialEq)]
pub enum MapTool {
    #[default]
    Pan,
    Select,
    Remove,
    AddReceiver,
    AddTransmitter,
    AddWall,
    Move,
    Zoom,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum MapAction {
    #[default]
    None,
    AddingConvexHull(Vec<[f32; 2]>),
    Moving(usize),
}

impl MapTool {
    pub const ALL: &'static [MapTool] = &[
        MapTool::Pan,
        MapTool::Select,
        MapTool::Remove,
        MapTool::AddReceiver,
        MapTool::AddTransmitter,
        MapTool::AddWall,
        MapTool::Move,
        MapTool::Zoom,
    ];

    pub fn interact(state: &mut TTSState, ui: &mut egui::Ui) {
        match state.map.map_selected_tool {
            MapTool::Zoom => ui.input(|input_state: &egui::InputState| {
                if input_state.pointer.primary_down() {
                    let pointer_delta: [f32; 2] = input_state.pointer.delta().into();
                    state.map.zoom *= 1.0 + (pointer_delta[1] * -0.01);
                    if state.map.zoom < 0.1 {
                        state.map.zoom = 0.1;
                    }
                }
            }),

            MapTool::Move => ui.input(|input_state: &egui::InputState| {
                if input_state.pointer.primary_pressed()
                    && let Some(pointer_position) = input_state.pointer.interact_pos()
                {
                    let pointer_coordinates: [f32; 2] = pointer_position.into();
                    let world_position: [f32; 2] = state.map.screen_to_world(pointer_coordinates);
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
                    let pointer_delta: [f32; 2] = input_state.pointer.delta().into();
                    let world_delta: [f32; 2] = [
                        pointer_delta[0] / state.map.zoom,
                        pointer_delta[1] / state.map.zoom,
                    ];

                    state.scene.move_object(object_index, world_delta);
                }

                if input_state.pointer.primary_released() {
                    state.map.action_in_progress = MapAction::None;
                }
            }),

            MapTool::Pan => ui.input(|input_state: &egui::InputState| {
                if input_state.pointer.primary_down() {
                    let pointer_delta: [f32; 2] = input_state.pointer.delta().into();
                    state.map.pan[0] += pointer_delta[0];
                    state.map.pan[1] += pointer_delta[1];
                }
            }),

            MapTool::Select => ui.input(|input_state: &egui::InputState| {
                if input_state.pointer.primary_clicked()
                    && let Some(pointer_position) = input_state.pointer.interact_pos()
                {
                    let pointer_coordinates: [f32; 2] = pointer_position.into();
                    let world_position: [f32; 2] = state.map.screen_to_world(pointer_coordinates);
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
                    let pointer_coordinates: [f32; 2] = pointer_position.into();
                    let world_position: [f32; 2] = state.map.screen_to_world(pointer_coordinates);
                    let search_radius: f32 = state.map.interaction_radius / state.map.zoom;
                    if let Some(object_index) = state
                        .scene
                        .find_object_index_around(world_position, search_radius)
                    {
                        state.scene.remove_object(object_index);
                    }
                }
            }),

            MapTool::AddReceiver => ui.input(|input_state: &egui::InputState| {
                if input_state.pointer.primary_clicked()
                    && let Some(pointer_position) = input_state.pointer.interact_pos()
                {
                    let pointer_coordinates: [f32; 2] = pointer_position.into();
                    let world_position: [f32; 2] = state.map.screen_to_world(pointer_coordinates);
                    state.scene.add_object(SceneObject::Receiver(Box::new(Receiver {
                        position: world_position,
                        style: state.map.style.receiver.clone(),
                    })));
                }
            }),

            MapTool::AddTransmitter => ui.input(|input_state: &egui::InputState| {
                if input_state.pointer.primary_clicked()
                    && let Some(pointer_position) = input_state.pointer.interact_pos()
                {
                    let pointer_coordinates: [f32; 2] = pointer_position.into();
                    let world_position: [f32; 2] = state.map.screen_to_world(pointer_coordinates);
                    state
                        .scene
                        .add_object(SceneObject::Transmitter(Box::new(Transmitter {
                            position: world_position,
                            sound_data: generate_sample_transmitter_sound(), 
                            style: state.map.style.transmitter.clone(),
                        })));
                }
            }),

            MapTool::AddWall => ui.input(|input_state: &egui::InputState| {
                if input_state.pointer.primary_clicked()
                    && let Some(pointer_position) = input_state.pointer.interact_pos()
                {
                    let pointer_coordinates: [f32; 2] = pointer_position.into();
                    let world_position: [f32; 2] = state.map.screen_to_world(pointer_coordinates);
                    match &mut state.map.action_in_progress {
                        MapAction::None => {
                            state.map.action_in_progress =
                                MapAction::AddingConvexHull(vec![world_position])
                        }
                        MapAction::AddingConvexHull(vertices) => {
                            vertices.push(world_position)
                        }
                        MapAction::Moving(_) => {}
                    }
                }

                if input_state.focused && input_state.key_pressed(egui::Key::Enter) {
                    let completed_action: MapAction = std::mem::replace(
                        &mut state.map.action_in_progress,
                        MapAction::None,
                    );

                    if let MapAction::AddingConvexHull(vertices) = completed_action
                        && vertices.len() >= 3
                    {
                        state.scene.add_object(SceneObject::Wall(Box::new(Wall {
                            verticies: vertices,
                            face_style: state.map.style.wall_face.clone(),
                            vertex_style: state.map.style.wall_vertex.clone(),
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
    pub pan: [f32; 2],
}

impl Default for MapState {
    fn default() -> Self {
        Self {
            map_selected_tool: MapTool::default(),
            action_in_progress: MapAction::default(),
            selected_object_index: None,
            settings: MapSettings::default(),
            zoom: 1.0,
            pan: [0.0, 0.0],
        }
    }
}

impl MapState {
    pub fn world_to_screen(&self, world_position: [f32; 2]) -> [f32; 2] {
        [
            (world_position[0] * self.zoom) + self.pan[0],
            (world_position[1] * self.zoom) + self.pan[1],
        ]
    }

    pub fn screen_to_world(&self, screen_position: [f32; 2]) -> [f32; 2] {
        [
            (screen_position[0] - self.pan[0]) / self.zoom,
            (screen_position[1] - self.pan[1]) / self.zoom,
        ]
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
            MapTool::AddTransmitter => MAP_ADDTRANSMITTER_ICON.into(),
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
    let toolbar_position: [f32; 2] = (dock_rectangle.min + MAP_TOOLBAR_PADDING).into();

    main_view(state, ui);
    toolbar(state, ui, toolbar_position);
}

fn main_view(state: &mut TTSState, ui: &mut egui::Ui) {
    MapTool::interact(state, ui);

    let painter: &Painter = ui.painter();

    for scene_object in state.scene.objects() {
        match scene_object {
            SceneObject::Wall(wall) => {
                let points: Vec<egui::Pos2> = wall
                    .verticies
                    .iter()
                    .map(|point: &[f32; 2]| state.map.world_to_screen(*point).into())
                    .collect();
                painter.add(Shape::convex_polygon(
                    points,
                    wall.face_style.fill_color,
                    wall.face_style.border_stroke,
                ));

                for wall_vertex in &wall.verticies {
                    let screen_position: [f32; 2] = state.map.world_to_screen(*wall_vertex);
                    painter.add(Shape::circle_filled(
                        screen_position.into(),
                        wall.vertex_style.radius * state.map.zoom,
                        wall.vertex_style.color,
                    ));
                }
            }
            SceneObject::Receiver(receiver) => {
                let screen_position: [f32; 2] = state.map.world_to_screen(receiver.position);
                painter.add(Shape::circle_filled(
                    screen_position.into(),
                    receiver.style.radius * state.map.zoom,
                    receiver.style.color,
                ));
            }
            SceneObject::Transmitter(transmitter) => {
                let screen_position: [f32; 2] = state.map.world_to_screen(transmitter.position);
                painter.add(Shape::circle_filled(
                    screen_position.into(),
                    transmitter.style.radius * state.map.zoom,
                    transmitter.style.color,
                ));
            }
        }
    }

    match &state.map.action_in_progress {
        MapAction::None => {}
        MapAction::AddingConvexHull(vertices) => {
            for wall_vertex in vertices {
                let screen_position: [f32; 2] = state.map.world_to_screen(*wall_vertex);
                painter.add(Shape::circle_filled(
                    screen_position.into(),
                    state.map.style.wall_vertex.radius * state.map.zoom,
                    state.map.style.wall_vertex.color,
                ));
            }
        }
        MapAction::Moving(_) => {}
    }
}

fn toolbar(state: &mut TTSState, ui: &mut egui::Ui, position: [f32; 2]) {
    egui::Area::new("MapToolbar".into())
        .fixed_pos(position)
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
