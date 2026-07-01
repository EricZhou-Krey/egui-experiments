use shared_view::Viewable;

#[derive(Default)]
pub struct Navigator;

impl Viewable for Navigator {
    fn title(&self) -> &str {
        "📊 Navigator"
    }
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Hello from Nav");
        // WORKING BRANCH
        // Current working idea:
        // Isometric landscape to right side where there are tech sci-y nodes that are hoverable to
        // to give a preview or static logo or etc for the project, connected with concepts and
        // stuff on the right
        // Settings to configure the angle, colour and etc of the scene
    }

    fn is_closeable(&self) -> bool {
        false
    }
}
