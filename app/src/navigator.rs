use crate::triangulation_graph::TriangulationGraph;
use shared_view::Viewable;

#[derive(Default)]
pub struct Navigator {
    triangulation_graph: TriangulationGraph,
}

impl Viewable for Navigator {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        self.triangulation_graph.draw_ui(ui);
    }
}
