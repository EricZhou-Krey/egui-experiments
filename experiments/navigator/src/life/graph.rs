#[derive(Default, Debug, Clone, PartialEq)]
pub struct LifeGraph {
    state: usize,
}

impl eframe::App for LifeGraph {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.heading("life graph");
    }

    fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {}
}
