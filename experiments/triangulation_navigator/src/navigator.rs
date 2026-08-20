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

    fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.triangulation_background.logic(ctx, frame);
    }
}

// TODO: after first experiment is completed add to this navigator the project ui and overlay, then
// make the navigator assign ids to the overlays and let the app choose which overlays correspond
// to each of the tab enums and then make it interactable as to which one is clicked on and etc
