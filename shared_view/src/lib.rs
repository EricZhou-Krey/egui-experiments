use egui::Ui;

pub trait Viewable {
    fn title(&self) -> &str;
    fn draw_ui(&mut self, ui: &mut Ui);
}