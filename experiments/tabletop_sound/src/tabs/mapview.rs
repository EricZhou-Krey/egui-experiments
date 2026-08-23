use crate::{
    state::TTSState,
    style_sheet::{MAP_CONVEXHULL_ICON, MAP_PAN_ICON, MAP_REMOVE_ICON, MAP_SELECT_ICON},
};

#[derive(Default, Debug, Clone, PartialEq)]
pub enum MapTool {
    #[default]
    Pan,
    Select,
    Remove,
    ConvexHull,
}

impl MapTool {
    pub const ALL: &'static [MapTool] = &[
        MapTool::Pan,
        MapTool::Select,
        MapTool::Remove,
        MapTool::ConvexHull,
    ];
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct MapState {
    pub map_tool: MapTool,
}

impl MapTool {
    pub fn icon(&self) -> egui::WidgetText {
        match self {
            MapTool::Pan => MAP_PAN_ICON.into(),
            MapTool::Select => MAP_SELECT_ICON.into(),
            MapTool::Remove => MAP_REMOVE_ICON.into(),
            MapTool::ConvexHull => MAP_CONVEXHULL_ICON.into(),
        }
    }
}

pub fn mapview_title(_state: &mut TTSState) -> egui::WidgetText {
    "MapView".into()
}

pub fn mapview_ui(state: &mut TTSState, ui: &mut egui::Ui) {
    let dock_rect: egui::Rect = ui.available_rect_before_wrap();
    let padding: egui::Vec2 = egui::vec2(8.0, 8.0);
    let toolbar_pos: egui::Pos2 = dock_rect.min + padding;

    ui.centered_and_justified(|ui| ui.heading("Map"));

    egui::Area::new("MapToolbar".into())
        .fixed_pos(toolbar_pos)
        .interactable(true)
        .show(ui, |ui| {
            egui::Frame::window(ui.style())
                .inner_margin(4.0)
                .corner_radius(6.0)
                .show(ui, |ui| {
                    let button_size: egui::Vec2 = egui::vec2(32.0, 32.0);
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
                                    .min_size(button_size),
                            )
                            .clicked()
                        {
                            state.map_state.map_tool = map_tool.clone();
                        }
                    }
                })
        });
}
