use crate::graph::GraphUpdate;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BoidGraph {
    state: usize,
}

impl BoidGraph {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<GraphUpdate> {
        ui.heading("boid_graph");
        None
    }

    pub fn logic(&mut self, _ctx: &egui::Context) {}
}
