use crate::triangulation_graph::TriangulationGraph;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Navigator {
    triangulation_background: TriangulationGraph,
}

impl Navigator {
    pub fn new() -> Self {
        Self {
            triangulation_background: TriangulationGraph::default(),
        }
    }

    pub fn draw_ui(&mut self, ui: &mut egui::Ui) {
        self.triangulation_background.draw_ui(ui);
    }
}
