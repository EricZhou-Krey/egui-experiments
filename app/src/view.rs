use eframe::egui;
use egui_dock::{DockArea, DockState, TabViewer};
use shared_view::viewable::Viewable;
use tabletop_sound::TabletopSoundTab;
use triangulation_navigator::navigator::Navigator;

struct Tab {
    label: String,
    view: Box<dyn Viewable>,
    is_closeable: bool, // Memory alignment :(
}

pub struct Viewer {
    tabs: Vec<Tab>,
}

impl TabViewer for Viewer {
    type Tab = usize;

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(*tab)
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        self.tabs[*tab].label.clone().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        self.tabs[*tab].view.draw_ui(ui);
    }

    fn is_closeable(&self, tab: &Self::Tab) -> bool {
        self.tabs[*tab].is_closeable
    }
}

pub struct View {
    dock_state: DockState<usize>,
    viewer: Viewer,
    navigator: Navigator,
}

impl Default for View {
    fn default() -> Self {
        macro_rules! create_tabs {
            [$(($t:ident, $is_closeable:expr)),* $(,)?] => {
                vec![
                    $(
                        Tab {
                            label: String::from(stringify!($t)),
                            view: Box::new(<$t>::new()) as Box<dyn Viewable>,
                            is_closeable: $is_closeable,
                        }
                    ),*
                ]
            };
        }

        let tabs: Vec<Tab> = create_tabs![(TabletopSoundTab, true)];

        let dock_state = DockState::new((0..tabs.len()).collect());

        Self {
            dock_state,
            viewer: Viewer { tabs },
            navigator: Navigator::new(),
        }
    }
}

// Next: Give access to navigator to edit the dockstate

impl eframe::App for View {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            if self.dock_state.main_surface().is_empty() {
                self.navigator.draw_ui(ui);
            }

            DockArea::new(&mut self.dock_state)
                .style(egui_dock::Style::from_egui(ui.style().as_ref()))
                .show_inside(ui, &mut self.viewer);
        });
    }
}
