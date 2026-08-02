use crate::triangulation_graph::TriangulationGraph;
use shared_view::Viewable;

#[derive(Default)]
pub struct Navigator {
    triangulation_graph: TriangulationGraph,
}

// PLAN:
// Selectable point on triangulation triangulation_graph
//      - Project mouse position to node that has been clicked
//      - Set velocity to 0
//      - Present preview at top left or right of the Project
//      - Title changes to description of the Project
//          - First example will be the tabletop sound Project

impl Navigator {
    fn handle_input(&mut self, _ui: &mut egui::Ui) {
        todo!();
    }
}

impl Viewable for Navigator {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        self.triangulation_graph.draw_ui(ui);
        egui::Frame::new()
            .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE))
            .fill(egui::Color32::from_gray(32))
            .inner_margin(8.0)
            .show(ui, |ui| {
                ui.label("This is inside a bordered box!");
            });
    }
}
