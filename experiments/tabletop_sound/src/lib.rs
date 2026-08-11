use shared_view::viewable::Viewable;

#[derive(Default)]
pub struct TabletopSoundTab {}

impl TabletopSoundTab {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Viewable for TabletopSoundTab {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Hello from TTS Test");
    }
}
