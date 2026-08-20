#[derive(Default, Debug, Clone, PartialEq)]
pub struct TabletopSoundTab {}

impl TabletopSoundTab {
    pub fn new() -> Self {
        Self::default()
    }
}

impl eframe::App for TabletopSoundTab {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Hello from TTS Test");
    }
}
