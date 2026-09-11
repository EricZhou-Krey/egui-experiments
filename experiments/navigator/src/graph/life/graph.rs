use crate::graph::GraphUpdate;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct LifeGraph {
    state: usize,
}

impl LifeGraph {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<GraphUpdate> {
        ui.heading("life graph");
        None
    }

    pub fn logic(&mut self, _ctx: &egui::Context) {}
}
