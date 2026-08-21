use crate::{state::TTSState, tab::Tab};
use egui_dock::{DockArea, DockState};

#[derive(Debug, Clone)]
pub struct TabletopSound {
    dock: DockState<Tab>,
    state: TTSState,
}

impl Default for TabletopSound {
    fn default() -> Self {
        Self {
            dock: TTSState::default_dock(),
            state: TTSState::default(),
        }
    }
}

impl eframe::App for TabletopSound {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui: &mut egui::Ui| {
            DockArea::new(&mut self.dock)
                .style(egui_dock::Style::from_egui(ui.style().as_ref()))
                .show_inside(ui, &mut self.state);
        });
    }
}
