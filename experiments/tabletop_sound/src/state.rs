use crate::tab::Tab;
use egui_dock::{DockState, NodeIndex, TabViewer, Tree};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TTSState {}

impl TTSState {
    pub fn default_dock() -> DockState<Tab> {
        let mut dock = DockState::new(vec![Tab::MapView]);

        let surface: &mut Tree<Tab> = dock.main_surface_mut();

        let map_node: NodeIndex = NodeIndex::root();

        let [left_pane, map_node]: [NodeIndex; 2] =
            surface.split_left(map_node, 0.2, vec![Tab::NodeTree]);

        let [map_node, _details_node]: [NodeIndex; 2] =
            surface.split_right(map_node, 0.75, vec![Tab::NodeDetails]);

        let [_map_node, _bottom_node]: [NodeIndex; 2] =
            surface.split_below(map_node, 0.7, vec![Tab::Console, Tab::SoundView]);

        let [_play_node, _tree_node]: [NodeIndex; 2] =
            surface.split_above(left_pane, 0.15, vec![Tab::PlayControls]);

        dock
    }
}

impl TabViewer for TTSState {
    type Tab = Tab;
    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(tab)
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Tab::Empty => "".into(),
            Tab::MapView => "Map".into(),
            Tab::NodeDetails => "NodeDetails".into(),
            Tab::Console => "Console".into(),
            Tab::NodeTree => "NodeTree".into(),
            Tab::SoundView => "Sound".into(),
            Tab::PlayControls => "PlayControls".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            Tab::Empty => {}
            Tab::MapView => {
                ui.centered_and_justified(|ui| ui.heading("Map"));
            }
            Tab::NodeDetails => {
                ui.centered_and_justified(|ui| ui.heading("NodeDetails"));
            }
            Tab::Console => {
                ui.centered_and_justified(|ui| ui.heading("Console"));
            }
            Tab::NodeTree => {
                ui.centered_and_justified(|ui| ui.heading("NodeTree"));
            }
            Tab::SoundView => {
                ui.centered_and_justified(|ui| ui.heading("Sound"));
            }
            Tab::PlayControls => {
                ui.centered_and_justified(|ui| ui.heading("PlayControls"));
            }
        }
    }
}
