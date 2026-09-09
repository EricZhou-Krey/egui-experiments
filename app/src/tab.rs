use egui::{Context, Id, RawInput, Ui, WidgetText};
use egui_dock::{DockState, TabViewer};
use std::ops::{Deref, DerefMut};
use tabletop_sound::tabletop_sound::TabletopSound;
use terminal::{
    file_system::{TerminalDirectory, TerminalFile},
    Terminal,
};

macro_rules! define_app_tabs {
    (
        pub enum $enum_name:ident {
            $( $variant:ident($inner:ty) => $title:expr ),* $(,)?
        }
    ) => {
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
            pub fn title(&self) -> WidgetText {
                match self {
                    Self::Empty => "".into(),
                    $( Self::$variant(_) => $title.into(), )*
                }
            }

            pub fn ui(&mut self, ui: &mut Ui) {
                match self {
                    Self::Empty => {}
                    $( Self::$variant(t) => { t.ui(ui); } )*
                }
            }

            pub fn logic(&mut self, ctx: &Context) {
                match self {
                    Self::Empty => {}
                    $( Self::$variant(t) => { t.logic(ctx); } )*
                }
            }

            pub fn raw_input_hook(&mut self, ctx: &Context, raw_input: &mut RawInput) {
                match self {
                    Self::Empty => {}
                    $( Self::$variant(t) => { t.raw_input_hook(ctx, raw_input); } )*
                }
            }
        }
    };
}

define_app_tabs! {
    pub enum Tab {
        TabletopSound(Box<TabletopSound>) => "TabletopSound",
        Terminal(Box<Terminal<TerminalFile, TerminalDirectory>>) => "Terminal",
    }
}

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

pub struct AppTabViewer;

impl TabViewer for AppTabViewer {
    type Tab = AppTab;

    fn id(&mut self, tab: &mut Self::Tab) -> Id {
        Id::new(tab.id)
    }

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.title()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        tab.ui(ui);
    }
}

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
