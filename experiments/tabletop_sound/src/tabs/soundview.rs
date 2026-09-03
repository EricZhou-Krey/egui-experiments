use crate::{scene::scene_object::SceneObject, state::TTSState};

pub fn soundview_title(_state: &mut TTSState) -> egui::WidgetText {
    "SoundView".into()
}

pub fn soundview_ui(state: &mut TTSState, ui: &mut egui::Ui) {
    ui.heading("Sound Data");
    ui.separator();

    if let Some(receiver_key) = state.map.selected_object_key {
        ui.label(format!("Listening at Receiver (ID: {:?})", receiver_key));
        ui.add_space(10.0);

        ui.label("Incoming Waveform:");

        let (rect, _response) = ui.allocate_exact_size(
            egui::vec2(ui.available_width(), 150.0),
            egui::Sense::hover(),
        );
        let painter = ui.painter();

        painter.rect_filled(rect, 4.0, ui.visuals().extreme_bg_color);

        let time = ui.input(|i| i.time);
        let num_points = 200;
        let mut points = Vec::with_capacity(num_points + 1);

        for i in 0..=num_points {
            let t = i as f32 / num_points as f32;
            let x = rect.left() + t * rect.width();

            let phase = time as f32 * 10.0 + t * std::f32::consts::TAU * 4.0;
            let y_offset = phase.sin();
            let y = rect.center().y + y_offset * (rect.height() * 0.4);

            points.push(egui::Pos2::new(x, y));
        }

        painter.add(egui::Shape::line(
            points,
            egui::Stroke::new(2.0, egui::Color32::LIGHT_BLUE),
        ));

        ui.ctx().request_repaint();

        ui.add_space(20.0);

        ui.heading("Emitter Contributions");
        ui.separator();

        let emitter_info: Vec<(usize, String)> = {
            state
                .view_scene()
                .objects()
                .enumerate()
                .filter(|(_, obj)| matches!(*obj, SceneObject::Emitter(_)))
                .map(|(i, _)| (i, format!("Emitter (ID: {})", i)))
                .collect()
        };

        if emitter_info.is_empty() {
            ui.label(egui::RichText::new("No emitters placed in the scene.").italics());
        } else {
            ui.indent("emitter_contributions_list", |ui| {
                for (i, label) in emitter_info {
                    ui.horizontal(|ui| {
                        ui.label(format!("🔊 {}", label));

                        let mock_volume = (i as f32 * 0.5 + time as f32).sin().abs() * 100.0;

                        ui.add(
                            egui::ProgressBar::new(mock_volume / 100.0)
                                .text(format!("{:.1}%", mock_volume)),
                        );
                    });
                }
            });
        }
    } else {
        ui.centered_and_justified(|ui| {
            ui.label(
                egui::RichText::new("No receiver selected.\nSelect a receiver in PlayControls to view incoming sound.")
                    .italics()
                    .color(egui::Color32::GRAY)
            );
        });
    }
}
