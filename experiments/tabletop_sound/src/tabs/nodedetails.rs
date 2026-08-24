use crate::state::TTSState;

pub fn nodedetails_title(_state: &mut TTSState) -> egui::WidgetText {
    "NodeDetails".into()
}

pub fn nodedetails_ui(state: &mut TTSState, ui: &mut egui::Ui) {
    let heading: String = if let Some(object_index) = state.map.selected_object_index {
        object_index.to_string()
    } else {
        "NodeDetails".into()
    };

    ui.centered_and_justified(|ui| ui.heading(heading));
}
