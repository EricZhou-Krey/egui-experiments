use crate::settings::{
    logic_sheet::MAP_INTERACTION_RADIUS,
    style::MapStyle,
    style_sheet::{MAP_GRID_HEIGHT, MAP_GRID_WIDTH},
};

pub mod logic_sheet;
pub mod style;
pub mod style_sheet;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SceneSettings {}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TTSSettings {}

#[derive(Debug, Clone, PartialEq)]
pub struct MapSettings {
    pub interaction_radius: f32,
    pub grid_cell_width: f32,
    pub grid_cell_height: f32,
    pub style: MapStyle,
}

impl Default for MapSettings {
    fn default() -> Self {
        Self {
            interaction_radius: MAP_INTERACTION_RADIUS,
            grid_cell_width: MAP_GRID_WIDTH,
            grid_cell_height: MAP_GRID_HEIGHT,
            style: MapStyle::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SoundSettings {
    volume: f32,
}

impl Default for SoundSettings {
    fn default() -> Self {
        Self { volume: 1.0 }
    }
}
