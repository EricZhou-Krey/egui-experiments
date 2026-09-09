#[derive(Debug, Default, Clone, PartialEq)]
pub struct BoidGraph {
    state: usize,
}

impl BoidGraph {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("boid_graph");
    }

    pub fn logic(&mut self, _ctx: &egui::Context) {}
}
