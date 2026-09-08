use crate::{
    layouts::{to_rect, UiFn},
    settings::style_sheet::LAYOUT_BLOCK_FRAME,
};
use egui::Rect;

pub const TITLE_RECT: (f32, f32, f32, f32) = (0.0, 0.0, 0.4, 0.3);
pub const BODY_RECT: (f32, f32, f32, f32) = (0.0, 0.3, 0.4, 1.0);

pub fn panels(max_rect: Rect) -> Vec<(Rect, UiFn)> {
    vec![
        (to_rect(TITLE_RECT, max_rect), title_content),
        (to_rect(BODY_RECT, max_rect), body_content),
    ]
}

fn title_content(ui: &mut egui::Ui) {
    egui::CentralPanel::default()
        .frame(LAYOUT_BLOCK_FRAME)
        .show(ui, |ui| {
            ui.centered_and_justified(|ui| ui.heading("TITLE TITLE"));
        });
}

fn body_content(ui: &mut egui::Ui) {
    egui::CentralPanel::default()
        .frame(LAYOUT_BLOCK_FRAME)
        .show(ui, |ui| {
            ui.centered_and_justified(|ui| ui.heading("TITLE BODY"));
        });
}
