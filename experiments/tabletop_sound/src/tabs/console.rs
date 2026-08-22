use crate::state::TTSState;

pub fn console_title(_state: &mut TTSState) -> egui::WidgetText {
    "Console".into()
}

pub fn console_ui(_state: &mut TTSState, ui: &mut egui::Ui) {
    ui.centered_and_justified(|ui| ui.heading("Console"));
}
