use crate::state::TTSState;

pub fn nodetree_title(_state: &mut TTSState) -> egui::WidgetText {
    "NodeTree".into()
}

pub fn nodetree_ui(_state: &mut TTSState, ui: &mut egui::Ui) {
    ui.centered_and_justified(|ui| ui.heading("NodeTree"));
}
