use crate::state::TTSState;

#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub enum Tab {
    #[default]
    Empty,
    MapView,
    NodeDetails,
    Console,
    NodeTree,
    SoundView,
    PlayControls,
}

impl Tab {
    pub const ALL: &'static [Tab] = &[
        Tab::MapView,
        Tab::NodeDetails,
        Tab::Console,
        Tab::NodeTree,
        Tab::SoundView,
        Tab::PlayControls,
    ];

    pub fn title(&self, _state: &mut TTSState) -> egui::WidgetText {
        match self {
            Tab::Empty => "".into(),
            Tab::MapView => "Map".into(),
            Tab::NodeDetails => "NodeDetails".into(),
            Tab::Console => "Console".into(),
            Tab::NodeTree => "NodeTree".into(),
            Tab::SoundView => "Sound".into(),
            Tab::PlayControls => "PlayControls".into(),
        }
    }

    pub fn ui(&mut self, _state: &mut TTSState, ui: &mut egui::Ui) {
        match self {
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

    pub fn logic(&mut self, _state: &mut TTSState, _ctx: &mut egui::Context) {
        match self {
            Tab::Empty => {}
            Tab::MapView => {}
            Tab::NodeDetails => {}
            Tab::Console => {}
            Tab::NodeTree => {}
            Tab::SoundView => {}
            Tab::PlayControls => {}
        }
    }
}
