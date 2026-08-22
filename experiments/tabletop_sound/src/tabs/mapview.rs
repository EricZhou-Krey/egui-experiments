use crate::state::TTSState;

pub fn mapview_title(_state: &mut TTSState) -> egui::WidgetText {
    "MapView".into()
}

pub fn mapview_ui(_state: &mut TTSState, ui: &mut egui::Ui) {
    ui.centered_and_justified(|ui| ui.heading("Map"));
}
