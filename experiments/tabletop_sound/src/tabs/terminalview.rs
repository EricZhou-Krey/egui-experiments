use crate::state::TTSState;

pub fn terminal_title(_state: &mut TTSState) -> egui::WidgetText {
    "Terminal".into()
}

pub fn terminal_ui(state: &mut TTSState, ui: &mut egui::Ui) {
    if let Some((command_fn, args)) = state.terminal.ui(ui) {
        command_fn(state, &args);
    }
}
