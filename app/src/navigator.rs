use shared_view::Viewable;

#[derive(Default)]
pub struct Navigator;

impl Viewable for Navigator {
    fn title(&self) -> &str {
        "📊 Navigator"
    }
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Hello from Nav");
    }
}
