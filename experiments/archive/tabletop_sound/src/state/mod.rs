pub mod map;

use crate::settings::TTSSettings;
use crate::sound::SoundState;
use crate::state::map::MapState;
use crate::tabs::Tab;
use crate::terminal::{create_tts_terminal, TTSTerminal};
use crate::{
    scene::Scene,
    settings::style_sheet::{LEFT_PANEL_WIDTH, TOP_LEFT_PANEL_HEIGHT, TOP_RIGHT_PANEL_HEIGHT},
};
use egui_dock::{DockState, NodeIndex, TabViewer, Tree};
use std::ops::{Deref, DerefMut};

pub struct TTSState {
    pub scene: Scene,
    pub map: MapState,
    pub terminal: TTSTerminal,
    pub sound: SoundState,
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

impl Default for TTSState {
    fn default() -> Self {
        Self::new()
    }
}

impl TTSState {
    pub fn new() -> Self {
        Self {
            scene: Scene::default(),
            map: MapState::default(),
            terminal: create_tts_terminal(),
            sound: SoundState::default(),
            settings: TTSSettings::default(),
        }
    }

    pub fn default_dock() -> DockState<Tab> {
        let mut dock: DockState<Tab> = DockState::new(vec![Tab::MapView]);

        let surface: &mut Tree<Tab> = dock.main_surface_mut();

        let root_panel: NodeIndex = NodeIndex::root();

        let [right_panel, left_panel]: [NodeIndex; 2] =
            surface.split_left(root_panel, LEFT_PANEL_WIDTH, vec![Tab::NodeDetails]);

        let [_map_panel, _console_sound_panel]: [NodeIndex; 2] = surface.split_below(
            right_panel,
            TOP_RIGHT_PANEL_HEIGHT,
            vec![Tab::Terminal, Tab::SoundView],
        );

        let [_play_node_details_panel, _node_tree_panel]: [NodeIndex; 2] = surface.split_below(
            left_panel,
            TOP_LEFT_PANEL_HEIGHT,
            vec![Tab::NodeTree, Tab::PlayControls],
        );

        dock
    }

    pub fn logic(&mut self, _ctx: &egui::Context) {}
}
