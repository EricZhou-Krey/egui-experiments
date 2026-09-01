use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{scene_object::SceneObject, state::TTSState};

pub fn playcontrols_title(_state: &mut TTSState) -> egui::WidgetText {
    "PlayControls".into()
}

pub fn playcontrols_ui(state: &mut TTSState, ui: &mut egui::Ui) {
    ui.heading("PlayControls");
    ui.separator();

    ui.horizontal(|ui| {
        ui.label("Receiver:");

        let receiver_indices: Vec<usize> = {
            let scene_objects: Ref<'_, Vec<Rc<RefCell<SceneObject>>>> = state.scene_objects();
            scene_objects
                .iter()
                .enumerate()
                .filter(|(_, obj)| matches!(*obj.borrow(), SceneObject::Receiver(_)))
                .map(|(i, _)| i)
                .collect()
        };

        let mut local_index: Option<usize> =
            state.sound.selected_receiver.and_then(|global_index| {
                receiver_indices
                    .iter()
                    .position(|&index| index == global_index)
            });

        let prev_local_index: Option<usize> = local_index;

        if ui.button("◀").clicked() && !receiver_indices.is_empty() {
            local_index = Some(
                local_index
                    .unwrap_or(0)
                    .checked_sub(1)
                    .unwrap_or(receiver_indices.len() - 1),
            );
        }

        egui::ComboBox::from_id_salt("receiver_select")
            .selected_text(match local_index {
                Some(index) => format!("Receiver {} (ID: {})", index + 1, receiver_indices[index]),
                None => "None".to_string(),
            })
            .show_ui(ui, |ui| {
                for (i, &global_index) in receiver_indices.iter().enumerate() {
                    ui.selectable_value(
                        &mut local_index,
                        Some(i),
                        format!("Receiver {} (ID: {})", i + 1, global_index),
                    );
                }
            });

        if ui.button("▶").clicked() && !receiver_indices.is_empty() {
            local_index = Some((local_index.unwrap_or(0) + 1) % receiver_indices.len());
        }

        if local_index != prev_local_index {
            if let Some(index) = local_index {
                state.sound.selected_receiver = Some(receiver_indices[index]);
            } else {
                state.sound.selected_receiver = None;
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
            state.sound.play_sound();
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
