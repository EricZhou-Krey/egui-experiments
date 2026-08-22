use crate::state::TTSState;

pub fn nodedetails_title(_state: &mut TTSState) -> egui::WidgetText {
    "NodeDetails".into()
}

pub fn nodedetails_ui(_state: &mut TTSState, ui: &mut egui::Ui) {
    ui.centered_and_justified(|ui| ui.heading("NodeDetails"));
}
