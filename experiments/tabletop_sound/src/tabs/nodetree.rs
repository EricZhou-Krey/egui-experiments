use crate::state::TTSState;

pub fn nodetree_title(_state: &mut TTSState) -> egui::WidgetText {
    "NodeTree".into()
}

pub fn nodetree_ui(state: &mut TTSState, ui: &mut egui::Ui) {
    ui.heading("Scene Tree");
    ui.separator();

    let object_info: Vec<(usize, String)> = {
        let scene_objects = state.scene_objects();
        scene_objects
            .iter()
            .enumerate()
            .map(|(i, obj)| {
                let borrowed_obj = obj.borrow();
                let display_name = match borrowed_obj.type_name() {
                    "wall" => "🧱 Wall",
                    "receiver" => "🎧 Receiver",
                    "emitter" => "🔊 Emitter",
                    _ => "❓ Unknown",
                };

                (i, format!("{}: {}", i, display_name))
            })
            .collect()
    };

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if object_info.is_empty() {
                ui.label(egui::RichText::new("Empty Scene").italics());
            } else {
                for (index, label_text) in object_info {
                    let is_selected = state.map.selected_object_index == Some(index);

                    let response = ui.selectable_label(is_selected, label_text);

                    if response.clicked() {
                        if is_selected {
                            state.map.selected_object_index = None;
                        } else {
                            state.map.selected_object_index = Some(index);
                        }
                    }
                }
            }
        });
}
