use eframe::egui;

#[derive(Default)]
pub struct View {
    views: Vec<Box<dyn Viewable>>,
    active_view_index: usize,
}

impl eframe::App for View {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("tab_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                for (index, view) in self.views.iter().enumerate() {
                    ui.selectable_value(&mut self.active_view_index, index, view.title());
                }
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            if let Some(view) = self.views.get_mut(self.active_view_index) {
                view.draw_ui(ui);
            }
        });
    }
}

pub trait Viewable {
    fn title(&self) -> &str;
    fn draw_ui(&mut self, ui: &mut egui::Ui);
}
