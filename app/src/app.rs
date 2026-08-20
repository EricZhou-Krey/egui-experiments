use crate::tab::{AppTabHandler, AppTabViewer};
use egui_dock::DockArea;
use triangulation_navigator::navigator::Navigator;

#[derive(Debug, Clone)]
pub struct App {
    tab_handler: AppTabHandler,
    navigator: Navigator,
}

impl Default for App {
    fn default() -> Self {
        Self {
            tab_handler: AppTabHandler::default(),
            navigator: Navigator::new(),
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui: &mut egui::Ui| {
            if self.tab_handler.dock.main_surface().is_empty()
                || self.tab_handler.dock.iter_all_tabs().next().is_none()
            {
                self.navigator.ui(ui, frame);
            }

            let mut tab_viewer: AppTabViewer = AppTabViewer { frame };

            DockArea::new(&mut self.tab_handler.dock)
                .style(egui_dock::Style::from_egui(ui.style().as_ref()))
                .show_inside(ui, &mut tab_viewer);
        });
    }

    fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Place to let navigator say whether to add ro remove tabs with custom impl
        for (_, tab) in self.tab_handler.dock.iter_all_tabs_mut() {
            tab.logic(ctx, frame);
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        for (_, tab) in self.tab_handler.dock.iter_all_tabs_mut() {
            tab.save(storage);
        }
    }

    fn raw_input_hook(&mut self, ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        for (_, tab) in self.tab_handler.dock.iter_all_tabs_mut() {
            tab.raw_input_hook(ctx, raw_input);
        }
    }
}
