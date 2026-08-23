use std::ops::{Deref, DerefMut};

use egui::{Painter, Shape};

use crate::{
    scene::{Receiver, Transmitter},
    state::TTSState,
    style::MapStyle,
    style_sheet::{   MAP_ADDWALL_ICON, 
        MAP_ADDRECEIVER_ICON, MAP_ADDTRANSMITTER_ICON, MAP_PAN_ICON,
        MAP_REMOVE_ICON, MAP_SELECT_ICON, MAP_TOOLBAR_BUTTON_SIZE, MAP_TOOLBAR_CORNER_RADIUS,
        MAP_TOOLBAR_MARGIN, MAP_TOOLBAR_PADDING,
    },
};

#[derive(Default, Debug, Clone, PartialEq)]
pub enum MapTool {
    #[default]
    Pan,
    Select,
    Remove,
    AddReceiver,
    AddTransmitter,
    AddWall,
}

impl MapTool {
    pub const ALL: &'static [MapTool] = &[
        MapTool::Pan,
        MapTool::Select,
        MapTool::Remove,
        MapTool::AddReceiver,
        MapTool::AddTransmitter,
        MapTool::AddWall,
    ];

    pub fn interact(state: &mut TTSState, ui: &mut egui::Ui) {
        match state.map_state.map_tool {
            MapTool::Pan => {}
            MapTool::Select => {}
            MapTool::Remove => {}
            MapTool::AddReceiver => ui.input(|ui| {
                if ui.pointer.primary_clicked() && let Some(position) = ui.pointer.interact_pos() {
                    state.scene.receivers.push(Receiver {
                        position: position.into(),
                        style: state.map_state.style.receiver.clone(),
                    });
                }
            }),
            MapTool::AddTransmitter => ui.input(|ui| {
                if ui.pointer.primary_clicked() && let Some(position) = ui.pointer.interact_pos() {
                    state.scene.transmitters.push(Transmitter {
                        position: position.into(),
                        style: state.map_state.style.transmitter.clone(),
                    });
                }
            }),
            MapTool::AddWall => {}
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct MapSettings {
    pub style: MapStyle,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct MapState {
    pub map_tool: MapTool,

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

    for wall in &state.scene.walls {
        painter.add(
            Shape::convex_polygon(
                wall.verticies.iter().map(|p| p.into()).collect(),
                wall.style.fill_color,
                wall.style.border_stroke,
            )
        );
    }

    for receiver in &state.scene.receivers {
        painter.add(
            Shape::circle_filled(
                receiver.position.into(),
                receiver.style.radius,
                receiver.style.color,
            )
        );
    }

    for transmitter in &state.scene.transmitters {
        painter.add(
            Shape::circle_filled(
                transmitter.position.into(),
                transmitter.style.radius,
                transmitter.style.color
            )
        );
    }
}

fn toolbar(state: &mut TTSState, ui: &mut egui::Ui, position: egui::Pos2) {
    egui::Area::new("MapToolbar".into())
        .fixed_pos(position)
        .interactable(true)
        .show(ui, |ui| {
            egui::Frame::window(ui.style())
                .inner_margin(MAP_TOOLBAR_MARGIN)
                .corner_radius(MAP_TOOLBAR_CORNER_RADIUS)
                .show(ui, |ui| {
                    let tool_selected: Vec<bool> = {
                        let mut tool_selected: Vec<bool> = vec![false; MapTool::ALL.len()];
                        tool_selected[state.map_state.map_tool.clone() as usize] = true;
                        tool_selected
                    };

                    for map_tool in MapTool::ALL {
                        if ui
                            .add(
                                egui::Button::new(map_tool.icon())
                                    .selected(tool_selected[map_tool.clone() as usize])
                                    .min_size(MAP_TOOLBAR_BUTTON_SIZE),
                            )
                            .clicked()
                        {
                            state.map_state.map_tool = map_tool.clone();
                        }
                    }
                })
        });
}
