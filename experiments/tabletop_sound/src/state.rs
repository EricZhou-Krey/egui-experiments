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
        tab.title(self)
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.ui(self, ui);
    }
}
