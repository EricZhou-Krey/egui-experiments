use egui::Ui;

pub trait Viewable {
    fn title(&self) -> &str;
    fn draw_ui(&mut self, ui: &mut Ui);
    fn is_closeable(&self) -> bool {
        true
    }
}
