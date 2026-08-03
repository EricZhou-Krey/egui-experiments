use shared_view::Viewable;

#[derive(Default)]
pub struct TabletopSoundTab;

impl TabletopSoundTab {
    pub fn new() -> Self {
        Self
    }
}

impl Viewable for TabletopSoundTab {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Hello from TTS");
    }
}
