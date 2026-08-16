use eframe::egui;
use egui_dock::{DockArea, DockState, TabViewer};
use shared_view::viewable::Viewable;
use tabletop_sound::TabletopSoundTab;
use triangulation_navigator::navigator::Navigator;

#[derive(Default, Debug)]
pub enum TabView {
    #[default]
    None,
    Navigator(Box<Navigator>),
    TabletopSoundTab(Box<TabletopSoundTab>),
}

impl Viewable for TabView {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        match self {
            TabView::None => (),
            TabView::Navigator(t) => t.draw_ui(ui),
            TabView::TabletopSoundTab(t) => t.draw_ui(ui),
        }
    }
}

struct Tab {
    label: String,
    view: TabView,
    is_closeable: bool,
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

// EDITING BELOW, refactoring and considering tab handler to mantain the tabs in the dock
// locations, figuring out how this works

pub struct View {
    dock_state: DockState<usize>,
    viewer: Viewer,
    navigator: Navigator,
}

impl Default for View {
    fn default() -> Self {
        let tabs: Vec<Tab> = vec![
            Tab {
                label: String::from("Navigator"),
                view: TabView::Navigator(Box::new(Navigator::new())),
                is_closeable: true,
            },
            Tab {
                label: String::from("TabletopSound"),
                view: TabView::TabletopSoundTab(Box::new(TabletopSoundTab::new())),
                is_closeable: true,
            },
        ];

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

        ui.request_repaint();
    }
}
