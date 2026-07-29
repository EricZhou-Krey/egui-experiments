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

impl Viewable for Navigator {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        self.triangulation_graph.draw_ui(ui);
    }
}
