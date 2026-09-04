pub mod map;
pub mod sound;
pub mod terminal;
use crate::scene::{scene_editor::SceneEditor, scene_viewer::SceneViewer};
use crate::settings::TTSSettings;
use crate::state::{map::MapState, sound::SoundState, terminal::TTSTerminalState};
use crate::tabs::Tab;
use crate::{
    scene::Scene,
    settings::style_sheet::{LEFT_PANEL_WIDTH, TOP_LEFT_PANEL_HEIGHT, TOP_RIGHT_PANEL_HEIGHT},
};
use egui_dock::{DockState, NodeIndex, TabViewer, Tree};
use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct TTSState {
    scene: Scene,
    pub map: MapState,
    pub terminal: TTSTerminalState,
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

impl TTSState {
    pub fn edit_scene(&mut self) -> SceneEditor<'_> {
        SceneEditor {
            scene: &mut self.scene,
            terminal: &mut self.terminal,
            sound: &mut self.sound,
        }
    }

    pub fn view_scene(&self) -> SceneViewer<'_> {
        SceneViewer {
            scene: &self.scene,
            terminal: &self.terminal,
            sound: &self.sound,
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

    pub fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
}
