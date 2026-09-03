use crate::{
    scene::{scene_object::SceneObject, SceneObjectKey},
    state::TTSState,
};

pub fn nodetree_title(_state: &mut TTSState) -> egui::WidgetText {
    "NodeTree".into()
}

pub fn nodetree_ui(state: &mut TTSState, ui: &mut egui::Ui) {
    ui.heading("Scene Tree");
    ui.separator();

    let object_info: Vec<(SceneObjectKey, String)> = state
        .view_scene()
        .key_objects()
        .map(|(key, obj)| {
            let display_name = match obj {
                SceneObject::Wall(..) => "🧱 Wall",
                SceneObject::Receiver(..) => "🎧 Receiver",
                SceneObject::Emitter(..) => "🔊 Emitter",
            };

            (key, format!("(ID: {:?}): {}", key, display_name))
        })
        .collect();

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if object_info.is_empty() {
                ui.label(egui::RichText::new("Empty Scene").italics());
            } else {
                for (key, label_text) in object_info {
                    let is_selected = state.map.selected_object_key == Some(key);

                    let response = ui.selectable_label(is_selected, label_text);

                    if response.clicked() {
                        if is_selected {
                            state.map.selected_object_key = None;
                        } else {
                            state.map.selected_object_key = Some(key);
                        }
                    }
                }
            }
        });
}
