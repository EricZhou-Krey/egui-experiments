use crate::state::TTSState;

use crate::tabs::{
    console::{console_title, console_ui},
    mapview::{mapview_title, mapview_ui},
    nodedetails::{nodedetails_title, nodedetails_ui},
    nodetree::{nodetree_title, nodetree_ui},
    playcontrols::{playcontrols_title, playcontrols_ui},
    soundview::{soundview_title, soundview_ui},
};

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

    pub fn title(&self, state: &mut TTSState) -> egui::WidgetText {
        match self {
            Tab::Empty => "".into(),
            Tab::MapView => mapview_title(state),
            Tab::NodeDetails => nodedetails_title(state),
            Tab::Console => console_title(state),
            Tab::NodeTree => nodetree_title(state),
            Tab::SoundView => soundview_title(state),
            Tab::PlayControls => playcontrols_title(state),
        }
    }

    pub fn ui(&mut self, state: &mut TTSState, ui: &mut egui::Ui) {
        match self {
            Tab::Empty => {}
            Tab::MapView => mapview_ui(state, ui),
            Tab::NodeDetails => nodedetails_ui(state, ui),
            Tab::Console => console_ui(state, ui),
            Tab::NodeTree => nodetree_ui(state, ui),
            Tab::SoundView => soundview_ui(state, ui),
            Tab::PlayControls => playcontrols_ui(state, ui),
        }
    }
}
