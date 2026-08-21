use eframe::App;
use egui_dock::{DockState, TabViewer};
use std::ops::{Deref, DerefMut};
use tabletop_sound::TabletopSound;
use triangulation_navigator::navigator::Navigator;

macro_rules! define_app_tabs {
    (
        #[derive($($derive:ident),*)]
        pub enum $enum_name:ident {
            $( $variant:ident($inner:ty) => $title:expr ),* $(,)?
        }
    ) => {
        #[derive($($derive),*)]
        pub enum $enum_name {
            Empty,
            $( $variant($inner), )*
        }

        impl Default for $enum_name {
            fn default() -> Self {
                Self::Empty
            }
        }

        impl $enum_name {
            pub fn title(&self) -> egui::WidgetText {
                match self {
                    Self::Empty => "".into(),
                    $( Self::$variant(_) => $title.into(), )*
                }
            }
        }

        impl App for $enum_name {
            fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
                match self {
                    Self::Empty => {}
                    $( Self::$variant(t) => t.ui(ui, frame), )*
                }
            }

            fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
                match self {
                    Self::Empty => {}
                    $( Self::$variant(t) => t.logic(ctx, frame), )*
                }
            }

            fn save(&mut self, storage: &mut dyn eframe::Storage) {
                match self {
                    Self::Empty => {}
                    $( Self::$variant(t) => t.save(storage), )*
                }
            }

            fn raw_input_hook(&mut self, ctx: &egui::Context, raw_input: &mut egui::RawInput) {
                match self {
                    Self::Empty => {}
                    $( Self::$variant(t) => t.raw_input_hook(ctx, raw_input), )*
                }
            }
        }
    };
}

define_app_tabs! {
    #[derive(Debug, Clone, PartialEq)]
    pub enum Tab {
        Navigator(Box<Navigator>) => "Navigator",
        TabletopSound(Box<TabletopSound>) => "TabletopSound",
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppTab {
    pub id: usize,
    pub content: Tab,
}

impl Deref for AppTab {
    type Target = Tab;
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl DerefMut for AppTab {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}

pub struct AppTabViewer<'a> {
    pub frame: &'a mut eframe::Frame,
}

impl<'a> TabViewer for AppTabViewer<'a> {
    type Tab = AppTab;

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(tab.id)
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.title()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.ui(ui, self.frame)
    }
}

#[derive(Debug, Clone)]
pub struct AppTabHandler {
    pub dock: DockState<AppTab>,
    next_tab_id: usize,
}

impl Default for AppTabHandler {
    fn default() -> Self {
        Self {
            dock: DockState::new(Vec::new()),
            next_tab_id: 0,
        }
    }
}

impl AppTabHandler {
    pub fn add_tab(&mut self, content: Tab) {
        let id: usize = self.next_tab_id;
        self.next_tab_id += 1;

        let new_tab: AppTab = AppTab { id, content };

        self.dock.main_surface_mut().push_to_focused_leaf(new_tab);
    }
}
