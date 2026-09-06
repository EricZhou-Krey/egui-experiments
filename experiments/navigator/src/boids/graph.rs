#[derive(Debug, Default, Clone, PartialEq)]
pub struct BoidGraph {
    state: usize,
}

impl eframe::App for BoidGraph {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.heading("boid_graph");
    }

    fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {}
}
