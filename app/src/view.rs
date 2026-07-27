use crate::navigator::Navigator;
use eframe::egui;
use egui_dock::{DockArea, DockState, TabViewer};
use shared_view::Viewable;
use tabletop_sound::TabletopSoundTab;

pub struct Viewer {
    views: Vec<Box<dyn Viewable>>,
}

impl TabViewer for Viewer {
    type Tab = usize;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        self.views[*tab].title().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        self.views[*tab].draw_ui(ui);
    }

    fn is_closeable(&self, tab: &Self::Tab) -> bool {
        self.views[*tab].is_closeable()
    }
}

pub struct View {
    dock_state: DockState<usize>,
    viewer: Viewer,
}

impl Default for View {
    fn default() -> Self {
        macro_rules! box_vec {
            [$($t:ty),* $(,)?] => {
                vec![$(Box::new(<$t>::default()) as Box<dyn Viewable>),*]
            };
        }

        let views: Vec<Box<dyn Viewable>> = box_vec![Navigator];

        let dock_state = DockState::new((0..views.len()).collect());

        Self {
            dock_state,
            viewer: Viewer { views },
        }
    }
}

impl eframe::App for View {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        DockArea::new(&mut self.dock_state)
            .style(egui_dock::Style::from_egui(ui.style().as_ref()))
            .show_inside(ui, &mut self.viewer);

        ui.request_repaint();
    }
}
