use crate::state::TTSState;

pub fn terminal_title(_state: &mut TTSState) -> egui::WidgetText {
    "Terminal".into()
}

pub fn terminal_ui(state: &mut TTSState, ui: &mut egui::Ui) {
    state.terminal.ui(ui);
}
