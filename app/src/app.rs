// app.rs
use crate::tab::{AppTabHandler, AppTabViewer};
use eframe::CreationContext;
use egui::{CentralPanel, Context, RawInput, Ui};
use egui_dock::{DockArea, Style};
use navigator::navigator::Navigator;

pub struct App {
    tab_handler: AppTabHandler,
    navigator: Navigator,
}

impl App {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Self {
            tab_handler: AppTabHandler::default(),
            navigator: Navigator::new(),
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ui, |ui: &mut Ui| {
            if self.tab_handler.dock.main_surface().is_empty()
                || self.tab_handler.dock.iter_all_tabs().next().is_none()
            {
                self.navigator.ui(ui);
            }

            let mut tab_viewer: AppTabViewer = AppTabViewer;

            DockArea::new(&mut self.tab_handler.dock)
                .style(Style::from_egui(ui.style().as_ref()))
                .show_inside(ui, &mut tab_viewer);
        });
    }

    fn logic(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.navigator.logic(ctx);
        for (_, tab) in self.tab_handler.dock.iter_all_tabs_mut() {
            tab.logic(ctx);
        }
    }

    fn raw_input_hook(&mut self, ctx: &Context, raw_input: &mut RawInput) {
        self.navigator.raw_input_hook(ctx, raw_input);
        for (_, tab) in self.tab_handler.dock.iter_all_tabs_mut() {
            tab.raw_input_hook(ctx, raw_input);
        }
    }
}
