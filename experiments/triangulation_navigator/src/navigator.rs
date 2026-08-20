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
}

impl eframe::App for Navigator {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        self.triangulation_background.ui(ui, frame);
    }
}
