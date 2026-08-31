use crate::state::TTSState;

pub fn soundview_title(_state: &mut TTSState) -> egui::WidgetText {
    "SoundView".into()
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SoundSettings {}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SoundState {}

pub fn soundview_ui(_state: &mut TTSState, ui: &mut egui::Ui) {
    ui.centered_and_justified(|ui| ui.heading("SoundView"));
}
