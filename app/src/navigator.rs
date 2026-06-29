use crate::view::Viewable;

#[derive(Default)]
pub struct Navigator {}

impl Viewable for Navigator {
    fn title(&self) -> &str {
        "Navigator"
    }
    fn draw_ui(&mut self, _ui: &mut egui::Ui) {
        todo!()
    }
}
