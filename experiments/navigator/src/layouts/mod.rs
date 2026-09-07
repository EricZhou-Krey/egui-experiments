use crate::layouts::{example::example_layout_ui, navigator::navigator_layout_ui};

pub mod example;
pub mod navigator;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Layout {
    #[default]
    Example,
    Navigator,
}

impl Layout {
    pub fn anchor_point(&self) -> [f32; 2] {
        match self {
            Self::Example => [0.5, 0.5],
            Self::Navigator => [0.8, 0.2],
        }
    }

    pub fn ui(&self, ui: &mut egui::Ui) {
        match self {
            Self::Example => example_layout_ui(ui),
            Self::Navigator => navigator_layout_ui(ui),
        }
    }
}
