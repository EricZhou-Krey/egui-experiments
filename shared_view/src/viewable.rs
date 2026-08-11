use egui::Ui;

pub trait Viewable {
    fn draw_ui(&mut self, ui: &mut Ui);
}
