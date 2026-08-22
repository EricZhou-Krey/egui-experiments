use crate::state::TTSState;

pub fn playcontrols_title(_state: &mut TTSState) -> egui::WidgetText {
    "PlayControls".into()
}

pub fn playcontrols_ui(_state: &mut TTSState, ui: &mut egui::Ui) {
    ui.centered_and_justified(|ui| ui.heading("PlayControls"));
}
