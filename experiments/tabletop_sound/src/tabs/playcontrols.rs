use crate::{scene::scene_object::SceneObject, state::TTSState};

pub fn playcontrols_title(_state: &mut TTSState) -> egui::WidgetText {
    "PlayControls".into()
}

pub fn playcontrols_ui(state: &mut TTSState, ui: &mut egui::Ui) {
    ui.heading("PlayControls");
    ui.separator();

    ui.horizontal(|ui| {
        ui.label("Receiver:");

        let receiver_keys: Vec<_> = state
            .scene
            .objects
            .iter()
            .filter(|(_, obj)| matches!(**obj, SceneObject::Receiver(_)))
            .map(|(key, _)| key)
            .collect();

        let mut local_index = state
            .map
            .selected_object_key
            .and_then(|global_key| receiver_keys.iter().position(|&key| key == global_key));

        let prev_local_index = local_index;

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
            state.sound.seek_to(0.0);
        }

        if ui.button("⏵ Play").clicked() {
            if let Some(key) = state.map.selected_object_key {
                let receiver_pos = state.scene.objects.get(key).and_then(|obj| {
                    if let SceneObject::Receiver(r) = obj {
                        Some(r.shape.center())
                    } else {
                        None
                    }
                });

                if let Some(pos) = receiver_pos {
                    let descriptor = state.sound.generate_scene_descriptor(pos, &state.scene);

                    if let Some(SceneObject::Receiver(r)) = state.scene.objects.get_mut(key) {
                        r.sound_descriptor = Some(descriptor);
                    }

                    if let Some(SceneObject::Receiver(r)) = state.scene.objects.get(key) {
                        state.sound.play(r);
                    }
                }
            }
        }

        if ui.button("⏸ Pause").clicked() {
            state.sound.pause();
        }

        if ui.button("⏹ Stop").clicked() {
            state.sound.stop();
        }

        if ui.button("⏭ Forward").clicked() {
            state.sound.seek_by(1.0);
        }
    });

    ui.add_space(10.0);

    let mut progress: f32 =
        ui.data_mut(|d| *d.get_temp_mut_or_default::<f32>(egui::Id::new("timeline_progress")));

    if let Some(first_handle) = state.sound.active_handles.first() {
        progress = first_handle.position() as f32;
    }

    ui.horizontal(|ui| {
        ui.label("Timeline:");

        let slider = egui::Slider::new(&mut progress, 0.0..=10.0)
            .show_value(false)
            .trailing_fill(true);

        if ui.add(slider).changed() {
            state.sound.seek_to(progress as f64);
        }
    });

    ui.data_mut(|d| d.insert_temp(egui::Id::new("timeline_progress"), progress));
}
