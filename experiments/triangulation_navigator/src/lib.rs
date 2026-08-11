use shared_view::viewable::Viewable;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Navigator {}

impl Navigator {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Viewable for Navigator {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Hello from Nav Test");
    }
}
