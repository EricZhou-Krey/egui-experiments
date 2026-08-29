use crate::state::TTSState;

pub fn nodedetails_title(_state: &mut TTSState) -> egui::WidgetText {
    "NodeDetails".into()
}

pub fn nodedetails_ui(state: &mut TTSState, ui: &mut egui::Ui) {
    if let Some(object_index) = state.map.selected_object_index && let Some(scene_object) = state.scene_object(object_index) {

        ui.heading(format!("{:?}", scene_object));
    } else {
        ui.centered_and_justified(|ui| ui.heading("No Selection"));
    }
}
