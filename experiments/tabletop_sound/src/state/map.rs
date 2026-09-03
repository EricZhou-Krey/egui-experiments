use crate::{
    logic_sheet::{
        MAP_BASE_ZOOM, MAP_INTERACTION_RADIUS, MAP_ZOOM_LIMIT, MAP_ZOOM_SENSITIVITY,
        generate_sample_transmitter_sound},
        scene_object::{Emitter, Receiver, SceneObject, Shape, Wall},
        state::TTSState, style::{FaceStyle, LineStyle, MapStyle, PointStyle},
        style_sheet::{
        MAP_ADDEMITTER_ICON, MAP_ADDRECEIVER_ICON, MAP_ADDWALL_ICON, MAP_GRID_HEIGHT, MAP_GRID_LINE_COLOR_MULTIPLIER, MAP_GRID_LINE_WIDTH, MAP_GRID_MIN_SCREEN_SPACING, MAP_GRID_SCALE_FACTOR, MAP_GRID_TEXT_COLOR_MULTIPLIER, MAP_GRID_TEXT_OFFSET_X, MAP_GRID_TEXT_OFFSET_Y_X_AXIS, MAP_GRID_TEXT_OFFSET_Y_Y_AXIS, MAP_GRID_TEXT_SIZE, MAP_GRID_WIDTH, MAP_MOVE_ICON, MAP_PAN_ICON, MAP_REMOVE_ICON, MAP_SELECT_ICON, MAP_TOOLBAR_BUTTON_SIZE, MAP_TOOLBAR_CORNER_RADIUS, MAP_TOOLBAR_MARGIN, MAP_TOOLBAR_PADDING, MAP_ZOOM_ICON
    }
};
use glam::Vec2;
use std::{cell::{Ref, RefCell}, ops::{Deref, DerefMut}, rc::Rc};

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
        let response: egui::Response = ui.allocate_response(ui.available_size(), egui::Sense::click_and_drag());

        match state.map.map_selected_tool {
            MapTool::Zoom => {
                if response.dragged() {
                    let pointer_delta: egui::Vec2 = response.drag_delta();
                    state.map.zoom *= 1.0 + (pointer_delta.y * -MAP_ZOOM_SENSITIVITY);
                    if state.map.zoom < MAP_ZOOM_LIMIT {
                        state.map.zoom = MAP_ZOOM_LIMIT;
                    }
                }
            }

            MapTool::Move => {
                if response.drag_started()
                    && let Some(pointer_position) = response.interact_pointer_pos()
                {
                    let screen_position: Vec2 = Vec2::new(pointer_position.x, pointer_position.y);
                    let world_position: Vec2 = state.map.screen_to_world(screen_position);
                    let search_radius: f32 = state.map.interaction_radius / state.map.zoom;
                    if let Some(object_index) = state
                        .find_scene_object_index_around(world_position, search_radius)
                    {
                        state.map.action_in_progress = MapAction::Moving(object_index);
                    }
                }

                if response.dragged()
                    && let MapAction::Moving(object_index) = state.map.action_in_progress
                {
                    let pointer_delta: egui::Vec2 = response.drag_delta();
                    let delta_vec: Vec2 = Vec2::new(pointer_delta.x, pointer_delta.y);
                    let world_delta: Vec2 = delta_vec / state.map.zoom;

                    if let Some(scene_object) = state.scene_object(object_index) {
                        scene_object.borrow_mut().mut_shape().translate(world_delta);
                    }
                }

                if response.drag_stopped() {
                    state.map.action_in_progress = MapAction::None;
                }
            }

            MapTool::Pan => {
                if response.dragged() {
                    let pointer_delta: egui::Vec2 = response.drag_delta();
                    let delta_vec: Vec2 = Vec2::new(pointer_delta.x, pointer_delta.y);
                    state.map.pan += delta_vec;
                }
            }

            MapTool::Select => {
                if response.clicked()
                    && let Some(pointer_position) = response.interact_pointer_pos()
                {
                    let screen_position: Vec2 = Vec2::new(pointer_position.x, pointer_position.y);
                    let world_position: Vec2 = state.map.screen_to_world(screen_position);
                    let search_radius: f32 = state.map.interaction_radius / state.map.zoom;
                    state.map.selected_object_index = state
                        .find_scene_object_index_around(world_position, search_radius);
                }
            }

            MapTool::Remove => {
                if response.clicked()
                    && let Some(pointer_position) = response.interact_pointer_pos()
                {
                    let screen_position: Vec2 = Vec2::new(pointer_position.x, pointer_position.y);
                    let world_position: Vec2 = state.map.screen_to_world(screen_position);
                    let search_radius: f32 = state.map.interaction_radius / state.map.zoom;

                    if let Some(object_index) = state
                        .find_scene_object_index_around(world_position, search_radius)
                    {
                        state.remove_scene_object(object_index);
                    }
                }
            }

            MapTool::AddReceiver => {
                if response.clicked()
                    && let Some(pointer_position) = response.interact_pointer_pos()
                {
                    let screen_position: Vec2 = Vec2::new(pointer_position.x, pointer_position.y);
                    let world_position: Vec2 = state.map.screen_to_world(screen_position);


                    let n_objects: usize = state.scene_objects().len();
                    state.map.selected_object_index = Some(n_objects);
                    state.add_scene_object(SceneObject::Receiver(Receiver {
                        shape: Shape::Point(world_position, state.map.style.receiver.clone()),
                    }));
                }
            }

            MapTool::AddEmitter => {
                if response.clicked()
                    && let Some(pointer_position) = response.interact_pointer_pos()
                {
                    let screen_position: Vec2 = Vec2::new(pointer_position.x, pointer_position.y);
                    let world_position: Vec2 = state.map.screen_to_world(screen_position);

                    let n_objects: usize = state.scene_objects().len();
                    state.map.selected_object_index = Some(n_objects);
                    state.add_scene_object(SceneObject::Emitter(Box::new(Emitter {
                        shape: Shape::Point(world_position, state.map.style.transmitter.clone()),
                        sound_data: generate_sample_transmitter_sound(),
                    })));
                }
            }

            MapTool::AddWall => {
                if response.clicked()
                    && let Some(pointer_position) = response.interact_pointer_pos()
                {
                    let screen_position: Vec2 = Vec2::new(pointer_position.x, pointer_position.y);
                    let world_position: Vec2 = state.map.screen_to_world(screen_position);
                    match &mut state.map.action_in_progress {
                        MapAction::None => {
                            state.map.action_in_progress = MapAction::AddingPolygon(Shape::Polygon(
                                vec![world_position],
                                state.map.style.wall_face.clone(),
                                state.map.style.wall_line.clone(),
                                state.map.style.wall_vertex.clone(),
                            ));
                        }
                        MapAction::AddingPolygon(shape) => {
                            if let Shape::Polygon(vertices, ..) = shape {
                                vertices.push(world_position);
                            }
                        }
                        MapAction::Moving(_) => {}
                    }
                }

                if !ui.egui_wants_keyboard_input() {
                    ui.input(|input_state: &egui::InputState| {
                        if input_state.focused && input_state.key_pressed(egui::Key::Enter) {
                            let completed_action: MapAction = std::mem::replace(
                                &mut state.map.action_in_progress,
                                MapAction::None,
                            );

                            if let MapAction::AddingPolygon(shape) = completed_action {
                                let n_objects: usize = state.scene_objects().len();
                                state.map.selected_object_index = Some(n_objects);
                                state.add_scene_object(SceneObject::Wall(Wall { shape }));
                            }
                        }

                        if input_state.focused && input_state.key_pressed(egui::Key::Escape) {
                            state.map.action_in_progress = MapAction::None;
                        }
                    });
                }
            }
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
