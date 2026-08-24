use std::ops::{Deref, DerefMut};

use crate::{
    scene::Scene,
    style_sheet::{LEFT_PANEL_WIDTH, TOP_LEFT_PANEL_HEIGHT, TOP_RIGHT_PANEL_HEIGHT},
    tab::Tab,
    tabs::mapview::MapState,
};
use egui_dock::{DockState, NodeIndex, TabViewer, Tree};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TTSSettings {}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TTSState {
    pub scene: Scene,

    pub map: MapState,

    pub settings: TTSSettings,
}

impl Deref for TTSState {
    type Target = TTSSettings;
    fn deref(&self) -> &Self::Target {
        &self.settings
    }
}

impl DerefMut for TTSState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.settings
    }
}

impl TabViewer for TTSState {
    type Tab = Tab;
    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(tab)
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.title(self)
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.ui(self, ui);
    }
}

impl TTSState {
    pub fn default_dock() -> DockState<Tab> {
        let mut dock = DockState::new(vec![Tab::MapView]);

        let surface: &mut Tree<Tab> = dock.main_surface_mut();

        let root_panel: NodeIndex = NodeIndex::root();

        let [right_panel, left_panel]: [NodeIndex; 2] =
            surface.split_left(root_panel, LEFT_PANEL_WIDTH, vec![Tab::NodeDetails]);

        let [_map_panel, _console_sound_panel]: [NodeIndex; 2] = surface.split_below(
            right_panel,
            TOP_RIGHT_PANEL_HEIGHT,
            vec![Tab::Console, Tab::SoundView],
        );

        let [_play_node_details_panel, _node_tree_panel]: [NodeIndex; 2] = surface.split_below(
            left_panel,
            TOP_LEFT_PANEL_HEIGHT,
            vec![Tab::NodeTree, Tab::PlayControls],
        );

        dock
    }

    pub fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
}
