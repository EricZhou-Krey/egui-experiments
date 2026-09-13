use crate::{
    layouts::{to_rect, UiFn},
    settings::style_sheet::LAYOUT_BLOCK_FRAME,
};
use egui::Rect;

const PREVIEW_RECT: (f32, f32, f32, f32) = (0.0, 0.0, 0.5, 0.4);
const TITLE_RECT: (f32, f32, f32, f32) = (0.5, 0.0, 1.0, 0.3);
const BODY_RECT: (f32, f32, f32, f32) = (0.5, 0.3, 1.0, 1.0);

pub fn panels(max_rect: Rect) -> Vec<(Rect, UiFn)> {
    vec![
        (to_rect(PREVIEW_RECT, max_rect), preview_content),
        (to_rect(TITLE_RECT, max_rect), navigator_title),
        (to_rect(BODY_RECT, max_rect), body_content),
    ]
}

fn navigator_title(ui: &mut egui::Ui) {
    egui::CentralPanel::default()
        .frame(LAYOUT_BLOCK_FRAME)
        .show(ui, |ui| {
            ui.centered_and_justified(|ui| ui.heading("INTERACTIVE NAVIGATOR"));
        });
}

fn body_content(ui: &mut egui::Ui) {
    egui::CentralPanel::default()
        .frame(LAYOUT_BLOCK_FRAME)
        .show(ui, |ui| {
            egui::ScrollArea::vertical().id_salt("navigator_body_scroll").show(ui, |ui| {
                ui.heading("SYSTEM OVERVIEW");
                ui.separator();
                
                ui.label("The Navigator is the central orchestrator of this application. It manages state transitions between different UI layout overlays and delegates input to the active background simulation.");
                ui.add_space(10.0);
                
                ui.label("Rather than relying on standard static buttons, this app utilizes spatial selection logic. Node coordinates are dynamically mapped from virtual UV space to screen space.");
                ui.add_space(10.0);
                
                ui.label(egui::RichText::new("Occlusion Avoidance").strong());
                ui.label("To prevent interactable nodes from being hidden behind opaque UI panels, the graphs continuously calculate bounding boxes. Nodes mathematically 'dodge' active layouts by teleporting to valid UV coordinates when clipped.");
            });
        });
}

fn preview_content(ui: &mut egui::Ui) {
    egui::CentralPanel::default()
        .frame(LAYOUT_BLOCK_FRAME)
        .show(ui, |ui| {
            egui::ScrollArea::vertical().id_salt("navigator_preview_scroll").show(ui, |ui| {
                ui.heading("BACKGROUND VISUALIZERS");
                ui.separator();
                
                ui.label(egui::RichText::new("1. Triangulation (Default)").strong());
                ui.label("A dynamic web of interconnected vertices with animated half-edges. Specific structural vertices are exposed as clickable navigation points.");
                
                ui.add_space(10.0);
                
                ui.label(egui::RichText::new("2. Boids Flocking").strong());
                ui.label("An implementation of Craig Reynolds' Boids algorithm. Agents continuously balance separation, alignment, and cohesion forces. The UI nodes attach to specific leaders in the flock.");
                
                ui.add_space(10.0);
                
                ui.label(egui::RichText::new("3. Game of Life").strong());
                ui.label("Conway's cellular automaton running on a dense grayscale grid. The interactable layout nodes act as continuous 'spawners', actively breathing life into the cells directly beneath them.");
            });
        });
}
