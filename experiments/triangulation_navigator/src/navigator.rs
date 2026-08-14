use crate::triangulation_graph::TriangulationGraph;
use shared_view::viewable::Viewable;

#[derive(Debug, Default, Clone)]
pub struct Navigator {
    triangulation_background: TriangulationGraph,
}

impl Navigator {
    pub fn new() -> Self {
        Self {
            triangulation_background: TriangulationGraph::default(),
        }
    }
}

impl Viewable for Navigator {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        self.triangulation_background.draw_ui(ui);
    }
}
