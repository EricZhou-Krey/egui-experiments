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
            ui.centered_and_justified(|ui| ui.heading("EGUI EXPERIMENT HUB"));
        });
}

fn body_content(ui: &mut egui::Ui) {
    egui::CentralPanel::default()
        .frame(LAYOUT_BLOCK_FRAME)
        .show(ui, |ui| {
            egui::ScrollArea::vertical().id_salt("title_body_scroll").show(ui, |ui| {
                ui.heading("WELCOME");
                ui.separator();
                
                ui.label("Welcome to this interactive UI and algorithmic experiment showcase, built entirely in Rust using the egui immediate-mode GUI library.");
                ui.add_space(10.0);
                
                ui.label("This environment serves as a playground for various custom systems, ranging from terminal emulators to artificial life simulations, all running concurrently within a unified, animated interface.");
                ui.add_space(10.0);
                
                ui.label(egui::RichText::new("How to explore:").strong());
                ui.label("• Hover and click on the highlighted nodes floating in the background graph to seamlessly transition between different layout modules.");
                ui.label("• Alternatively, use the integrated terminal at the bottom of the screen to execute commands, swap visualizer modes, and navigate the app programmatically.");
            });
        });
}
