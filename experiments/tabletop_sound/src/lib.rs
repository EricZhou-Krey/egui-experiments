#[derive(Default, Debug, Clone, PartialEq)]
pub struct TabletopSoundTab {}

impl TabletopSoundTab {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TabletopSoundTab {
    pub fn draw_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Hello from TTS Test");
    }
}
