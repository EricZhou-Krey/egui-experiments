use crate::triangulation_graph::TriangulationGraph;
use shared_view::Viewable;

#[derive(Default)]
enum Overlay {
    #[default]
    Navigator,

    ExampleNodeOne,
    ExampleNodeTwo,
}

impl Overlay {
    fn draw_overlay(&self, ui: &mut egui::Ui) {
        match self {
            Overlay::Navigator => {
                egui::Frame::new()
                    .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE))
                    .fill(egui::Color32::BLACK)
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        ui.label("This is the navigator overlay");
                    });
            }
            Overlay::ExampleNodeOne => {
                egui::Frame::new()
                    .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE))
                    .fill(egui::Color32::BLACK)
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        ui.label("This is the example one overlay");
                    });
            }
            Overlay::ExampleNodeTwo => {
                egui::Frame::new()
                    .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE))
                    .fill(egui::Color32::BLACK)
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        ui.label("This is the example two overlay");
                    });
            }
        }
    }
}

#[derive(Default)]
pub struct Navigator {
    triangulation_graph: TriangulationGraph,
    overlay: Overlay,
}

// PLAN:
// Selectable point on triangulation triangulation_graph
//      - Project mouse position to node that has been clicked
//      - Set velocity to 0
//      - Present preview at top left or right of the Project
//      - Title changes to description of the Project
//          - First example will be the tabletop sound Project

impl Navigator {
    fn handle_input(&mut self, ui: &mut egui::Ui) {
        ui.input(|i| {
            for key in &i.keys_down {
                match key {
                    egui::Key::Num1 => self.overlay = Overlay::Navigator,
                    egui::Key::Num2 => self.overlay = Overlay::ExampleNodeOne,
                    egui::Key::Num3 => self.overlay = Overlay::ExampleNodeTwo,
                    _ => {}
                }
            }
        });
    }
}

impl Viewable for Navigator {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        self.handle_input(ui);
        self.triangulation_graph.draw_ui(ui);
        self.overlay.draw_overlay(ui);
    }
}
