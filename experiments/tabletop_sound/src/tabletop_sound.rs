use crate::{state::TTSState, tabs::Tab};
use egui_dock::{DockArea, DockState};

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
        egui::Panel::top("DockerBar").show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Reset").clicked() {
                    self.dock = TTSState::default_dock();
                }
                for tab in Tab::ALL {
                    let is_open = self.dock.find_tab(tab).is_some();

                    if ui
                        .selectable_label(is_open, tab.title(&mut self.state))
                        .clicked()
                    {
                        if let Some(locator) = self.dock.find_tab(tab) {
                            self.dock.remove_tab(locator);
                        } else {
                            self.dock
                                .main_surface_mut()
                                .push_to_focused_leaf(tab.clone());
                        }
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ui, |ui: &mut egui::Ui| {
            DockArea::new(&mut self.dock)
                .style(egui_dock::Style::from_egui(ui.style().as_ref()))
                .show_inside(ui, &mut self.state);
        });
    }

    fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.state.logic(ctx, frame);
    }
}
