use crate::{
    layouts::{to_rect, UiFn},
    settings::style_sheet::LAYOUT_BLOCK_FRAME,
};
use egui::Rect;

const TITLE_RECT: (f32, f32, f32, f32) = (0.0, 0.0, 0.4, 0.2);
const DESC_RECT: (f32, f32, f32, f32) = (0.0, 0.2, 0.4, 1.0);
const PREVIEW_RECT: (f32, f32, f32, f32) = (0.4, 0.0, 1.0, 0.5);

pub fn panels(max_rect: Rect) -> Vec<(Rect, UiFn)> {
    vec![
        (to_rect(TITLE_RECT, max_rect), title_content),
        (to_rect(DESC_RECT, max_rect), description_content),
        (to_rect(PREVIEW_RECT, max_rect), preview_content),
    ]
}

fn title_content(ui: &mut egui::Ui) {
    egui::CentralPanel::default()
        .frame(LAYOUT_BLOCK_FRAME)
        .show(ui, |ui| {
            ui.centered_and_justified(|ui| ui.heading("TERMINAL EMULATOR"));
        });
}

fn description_content(ui: &mut egui::Ui) {
    egui::CentralPanel::default()
        .frame(LAYOUT_BLOCK_FRAME)
        .show(ui, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading("PROJECT PURPOSE:\nA custom terminal emulator...")
            });
        });
}

fn preview_content(ui: &mut egui::Ui) {
    egui::CentralPanel::default()
        .frame(LAYOUT_BLOCK_FRAME)
        .show(ui, |ui| {
            ui.centered_and_justified(|ui| ui.heading("[ TERMINAL PREVIEW UI GOES HERE ]"));
        });
}
