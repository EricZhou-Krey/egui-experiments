use crate::{
    logic_sheet::MAP_INTERACTION_RADIUS, scene::{Receiver, SceneObject, Transmitter, Wall}, state::TTSState, style::MapStyle, style_sheet::{MAP_MOVE_ICON, 
        MAP_ADDRECEIVER_ICON, MAP_ADDTRANSMITTER_ICON, MAP_ADDWALL_ICON, MAP_PAN_ICON,
        MAP_REMOVE_ICON, MAP_SELECT_ICON, MAP_TOOLBAR_BUTTON_SIZE, MAP_TOOLBAR_CORNER_RADIUS,
        MAP_TOOLBAR_MARGIN, MAP_TOOLBAR_PADDING,
    }
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
    ];

    pub fn interact(state: &mut TTSState, ui: &mut egui::Ui) {
        match state.map.map_selected_tool {
            MapTool::Move => ui.input(|input_state: &egui::InputState| {
                if input_state.pointer.primary_pressed()
                    && let Some(pointer_position) = input_state.pointer.interact_pos()
                    && let Some(object_index) = state.scene.find_object_index_around(pointer_position.into(), state.map.interaction_radius)
                {
                    state.map.action_in_progress = MapAction::Moving(object_index);
                }

                if input_state.pointer.primary_down()
                    && let MapAction::Moving(object_index) = state.map.action_in_progress
                    && let Some(scene_object) = state.scene.objects.get_mut(object_index)
                {
                    let pointer_delta: egui::Vec2 = input_state.pointer.delta();
                    match scene_object {
                        SceneObject::Wall(wall) => {
                            for vertex in &mut wall.verticies {
                                vertex[0] += pointer_delta.x;
                                vertex[1] += pointer_delta.y;
                            }
                        }
                        SceneObject::Receiver(receiver) => {
                            receiver.position[0] += pointer_delta.x;
                            receiver.position[1] += pointer_delta.y;
                        }
                        SceneObject::Transmitter(transmitter) => {
                            transmitter.position[0] += pointer_delta.x;
                            transmitter.position[1] += pointer_delta.y;
                        }
                    }
                }

                if input_state.pointer.primary_released() {
                    state.map.action_in_progress = MapAction::None;
                }
            }),
            MapTool::Pan => ui.input(|input_state: &egui::InputState| {
                if input_state.pointer.primary_down() {
                    let pointer_delta: egui::Vec2 = input_state.pointer.delta();
                    for scene_object in &mut state.scene.objects {
                        match scene_object {
                            SceneObject::Wall(wall) => {
                                for vertex in &mut wall.verticies {
                                    vertex[0] += pointer_delta.x;
                                    vertex[1] += pointer_delta.y;
                                }
                            }
                            SceneObject::Receiver(receiver) => {
                                receiver.position[0] += pointer_delta.x;
                                receiver.position[1] += pointer_delta.y;
                            }
                            SceneObject::Transmitter(transmitter) => {
                                transmitter.position[0] += pointer_delta.x;
                                transmitter.position[1] += pointer_delta.y;
                            }
                        }
                    }
                }
            }),
            MapTool::Select => ui.input(|input_state: &egui::InputState| {
                if input_state.pointer.primary_clicked()
                    && let Some(pointer_position) = input_state.pointer.interact_pos()
                {
                    state.map.selected_object_index = state.scene.find_object_index_around(pointer_position.into(), state.map.interaction_radius); 
                }
            }),
            MapTool::Remove => ui.input(|input_state: &egui::InputState| {
                if input_state.pointer.primary_clicked()
                    && let Some(pointer_position) = input_state.pointer.interact_pos()
                    && let Some(object_index) = state.scene.find_object_index_around(pointer_position.into(), state.map.interaction_radius)
                {
                    state.scene.objects.remove(object_index);
                }
            }),
            MapTool::AddReceiver => ui.input(|input_state: &egui::InputState| {
                if input_state.pointer.primary_clicked()
                    && let Some(pointer_position) = input_state.pointer.interact_pos()
                {
                    state.scene.objects.push(SceneObject::Receiver(Receiver {
                        position: pointer_position.into(),
                        style: state.map.style.receiver.clone(),
                    }));
                }
            }),
            MapTool::AddTransmitter => ui.input(|input_state: &egui::InputState| {
                if input_state.pointer.primary_clicked()
                    && let Some(pointer_position) = input_state.pointer.interact_pos()
                {
                    state
                        .scene
                        .objects
                        .push(SceneObject::Transmitter(Transmitter {
                            position: pointer_position.into(),
                            style: state.map.style.transmitter.clone(),
                        }));
                }
            }),
            MapTool::AddWall => ui.input(|input_state: &egui::InputState| {
                if input_state.pointer.primary_clicked()
                    && let Some(pointer_position) = input_state.pointer.interact_pos()
                {
                    match &mut state.map.action_in_progress {
                        MapAction::None => {
                            state.map.action_in_progress =
                                MapAction::AddingConvexHull(vec![pointer_position.into()])
                        }
                        MapAction::AddingConvexHull(vertices) => {
                            vertices.push(pointer_position.into())
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
                        state.scene.objects.push(SceneObject::Wall(Wall {
                            verticies: vertices,
                            face_style: state.map.style.wall_face.clone(),
                            vertex_style: state.map.style.wall_vertex.clone(),
                        }));
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

#[derive(Default, Debug, Clone, PartialEq)]
pub struct MapState {
    pub map_selected_tool: MapTool,
    pub action_in_progress: MapAction,
    pub selected_object_index: Option<usize>,
    pub settings: MapSettings,
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
        }
    }
}

pub fn mapview_title(_state: &mut TTSState) -> egui::WidgetText {
    "MapView".into()
}

pub fn mapview_ui(state: &mut TTSState, ui: &mut egui::Ui) {
    let dock_rect: egui::Rect = ui.available_rect_before_wrap();
    let toolbar_position: egui::Pos2 = dock_rect.min + MAP_TOOLBAR_PADDING;

    main_view(state, ui);
    toolbar(state, ui, toolbar_position);
}

fn main_view(state: &mut TTSState, ui: &mut egui::Ui) {
    MapTool::interact(state, ui);

    let painter: &Painter = ui.painter();

    for scene_object in &state.scene.objects {
        match scene_object {
            SceneObject::Wall(wall) => {
                let points: Vec<egui::Pos2> = wall
                    .verticies
                    .iter()
                    .map(|point: &[f32; 2]| (*point).into())
                    .collect();
                painter.add(Shape::convex_polygon(
                    points,
                    wall.face_style.fill_color,
                    wall.face_style.border_stroke,
                ));

                for wall_vertex in &wall.verticies {
                    painter.add(Shape::circle_filled(
                        (*wall_vertex).into(),
                        wall.vertex_style.radius,
                        wall.vertex_style.color,
                    ));
                }
            }
            SceneObject::Receiver(receiver) => {
                painter.add(Shape::circle_filled(
                    receiver.position.into(),
                    receiver.style.radius,
                    receiver.style.color,
                ));
            }
            SceneObject::Transmitter(transmitter) => {
                painter.add(Shape::circle_filled(
                    transmitter.position.into(),
                    transmitter.style.radius,
                    transmitter.style.color,
                ));
            }
        }
    }

    match &state.map.action_in_progress {
        MapAction::None => {}
        MapAction::AddingConvexHull(vertices) => {
            for wall_vertex in vertices {
                painter.add(Shape::circle_filled(
                    (*wall_vertex).into(),
                    state.map.style.wall_vertex.radius,
                    state.map.style.wall_vertex.color,
                ));
            }
        }
        MapAction::Moving(_) => {}
    }
}

fn toolbar(state: &mut TTSState, ui: &mut egui::Ui, position: egui::Pos2) {
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
