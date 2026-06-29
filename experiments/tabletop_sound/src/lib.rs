use shared_view::Viewable;

#[derive(Default)]
pub struct TabletopSoundTab;

impl Viewable for TabletopSoundTab {
    fn title(&self) -> &str {
        "TTS"
    }
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Hello from TTS");
    }
}
