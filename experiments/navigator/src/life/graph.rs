#[derive(Default, Debug, Clone, PartialEq)]
pub struct LifeGraph {
    state: usize,
}

impl LifeGraph {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("life graph");
    }

    pub fn logic(&mut self, _ctx: &egui::Context) {}
}
