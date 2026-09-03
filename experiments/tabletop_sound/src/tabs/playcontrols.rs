use crate::scene::{scene_object::SceneObject, SceneObjectKey};
use crate::state::TTSState;

pub fn playcontrols_title(_state: &mut TTSState) -> egui::WidgetText {
    "PlayControls".into()
}

pub fn playcontrols_ui(state: &mut TTSState, ui: &mut egui::Ui) {
    ui.heading("PlayControls");
    ui.separator();

    ui.horizontal(|ui| {
        ui.label("Receiver:");

        let receiver_keys: Vec<SceneObjectKey> = {
            let viewer = state.view_scene();
            viewer
                .scene
                .objects
                .iter()
                .filter(|(_, obj)| matches!(**obj, SceneObject::Receiver(_)))
                .map(|(key, _)| key)
                .collect()
        };

        let mut local_index: Option<usize> = state
            .map
            .selected_object_key
            .and_then(|global_key| receiver_keys.iter().position(|&key| key == global_key));

        let prev_local_index: Option<usize> = local_index;

        if ui.button("◀").clicked() && !receiver_keys.is_empty() {
            local_index = Some(
                local_index
                    .unwrap_or(0)
                    .checked_sub(1)
                    .unwrap_or(receiver_keys.len() - 1),
            );
        }

        egui::ComboBox::from_id_salt("receiver_select")
            .selected_text(match local_index {
                Some(index) => format!("Receiver {} (ID: {:?})", index + 1, receiver_keys[index]),
                None => "None".to_string(),
            })
            .show_ui(ui, |ui| {
                for (i, &key) in receiver_keys.iter().enumerate() {
                    ui.selectable_value(
                        &mut local_index,
                        Some(i),
                        format!("Receiver {} (ID: {:?})", i + 1, key),
                    );
                }
            });

        if ui.button("▶").clicked() && !receiver_keys.is_empty() {
            local_index = Some((local_index.unwrap_or(0) + 1) % receiver_keys.len());
        }

        if local_index != prev_local_index {
            if let Some(index) = local_index {
                state.map.selected_object_key = Some(receiver_keys[index]);
            } else {
                state.map.selected_object_key = None;
            }
        }
    });

    ui.add_space(10.0);

    ui.horizontal(|ui| {
        if ui.button("⏮ Back").clicked() {
            todo!();
        }

        if ui.button("⏵ Play").clicked() {
            //TODO
            state.sound.play_sound(state.map.selected_object_key);
        }

        if ui.button("⏹ Stop").clicked() {
            todo!();
        }

        if ui.button("⏭ Forward").clicked() {
            todo!();
        }
    });

    ui.add_space(10.0);

    let mut progress: f32 =
        ui.data_mut(|d| *d.get_temp_mut_or_default::<f32>(egui::Id::new("timeline_progress")));

    ui.horizontal(|ui| {
        ui.label("Timeline:");
        let slider = egui::Slider::new(&mut progress, 0.0..=100.0)
            .show_value(false)
            .trailing_fill(true);

        if ui.add(slider).changed() {
            todo!();
        }
    });

    ui.data_mut(|d| d.insert_temp(egui::Id::new("timeline_progress"), progress));
}
