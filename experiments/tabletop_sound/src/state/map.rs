use crate::{
    scene::{SceneObjectKey, scene_object::{Emitter, Receiver, SceneObject, Shape, Wall}}, settings::{
        MapSettings, logic_sheet::{
            MAP_BASE_ZOOM, MAP_ZOOM_LIMIT, MAP_ZOOM_SENSITIVITY, generate_sample_emitter_sound
        }, style::PointStyle, style_sheet::{
            MAP_ADDEMITTER_ICON, MAP_ADDRECEIVER_ICON, MAP_ADDWALL_ICON, MAP_MOVE_ICON, MAP_PAN_ICON, MAP_REMOVE_ICON,
            MAP_SELECT_ICON, MAP_ZOOM_ICON
        }
    }, state::TTSState
};
use glam::Vec2;

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
    Moving(SceneObjectKey),
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
                    let search_radius: f32 = state.map.settings.interaction_radius / state.map.zoom;
                    if let Some(object_key) = state.view_scene().key_object_around(world_position, search_radius).map(|kv| kv.0)
                    {
                        state.map.action_in_progress = MapAction::Moving(object_key);
                    }
                }

                if response.dragged()
                    && let MapAction::Moving(object_key) = state.map.action_in_progress
                {
                    let pointer_delta: egui::Vec2 = response.drag_delta();
                    let delta_vec: Vec2 = Vec2::new(pointer_delta.x, pointer_delta.y);
                    let world_delta: Vec2 = delta_vec / state.map.zoom;

                    state.edit_scene().modify_object(object_key, |object: &mut SceneObject| {
                        object.mut_shape().translate(world_delta);
                    });
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
                    let search_radius: f32 = state.map.settings.interaction_radius / state.map.zoom;
                    state.map.selected_object_key = state.view_scene().key_object_around(world_position, search_radius).map(|kv| kv.0);
                }
            }

            MapTool::Remove => {
                if response.clicked()
                    && let Some(pointer_position) = response.interact_pointer_pos()
                {
                    let screen_position: Vec2 = Vec2::new(pointer_position.x, pointer_position.y);
                    let world_position: Vec2 = state.map.screen_to_world(screen_position);
                    let search_radius: f32 = state.map.settings.interaction_radius / state.map.zoom;

                    if let Some(object_key) = state.view_scene().key_object_around(world_position, search_radius).map(|kv| kv.0)
                    {
                        state.edit_scene().remove_object(object_key);
                    }
                }
            }

            MapTool::AddReceiver => {
                if response.clicked()
                    && let Some(pointer_position) = response.interact_pointer_pos()
                {
                    let screen_position: Vec2 = Vec2::new(pointer_position.x, pointer_position.y);
                    let world_position: Vec2 = state.map.screen_to_world(screen_position);


                    let receiver_point_style: PointStyle = state.map.settings.style.receiver.clone();
                    state.map.selected_object_key = Some(state.edit_scene().add_object(SceneObject::Receiver(Receiver {
                        shape: Shape::Point(world_position, receiver_point_style),
                    })));
                }
            }

            MapTool::AddEmitter => {
                if response.clicked()
                    && let Some(pointer_position) = response.interact_pointer_pos()
                {
                    let screen_position: Vec2 = Vec2::new(pointer_position.x, pointer_position.y);
                    let world_position: Vec2 = state.map.screen_to_world(screen_position);

                    let emitter_point_style: PointStyle = state.map.settings.style.emitter.clone();
                    state.map.selected_object_key = Some(state.edit_scene().add_object(SceneObject::Emitter(Box::new(Emitter {
                        shape: Shape::Point(world_position, emitter_point_style),
                        sound_data: generate_sample_emitter_sound(),
                    }))));
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
                                state.map.settings.style.wall_face.clone(),
                                state.map.settings.style.wall_line.clone(),
                                state.map.settings.style.wall_vertex.clone(),
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
                                state.map.selected_object_key = Some(state.edit_scene().add_object(SceneObject::Wall(Wall { shape })));
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
    pub selected_object_key: Option<SceneObjectKey>,
    pub settings: MapSettings,
    pub zoom: f32,
    pub pan: Vec2,
}

impl Default for MapState {
    fn default() -> Self {
        Self {
            map_selected_tool: MapTool::default(),
            action_in_progress: MapAction::default(),
            selected_object_key: None,
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
