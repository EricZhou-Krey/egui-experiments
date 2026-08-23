use std::ops::{Deref, DerefMut};

use crate::style::{FaceStyle, PointStyle};

#[derive(Debug, Clone, PartialEq)]
pub struct Wall {
    pub verticies: Vec<[f32; 2]>,
    pub style: FaceStyle,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Receiver {
    pub position: [f32; 2],
    pub style: PointStyle,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Transmitter {
    pub position: [f32; 2],
    pub style: PointStyle,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SceneSettings;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Scene {
    pub walls: Vec<Wall>,
    pub receivers: Vec<Receiver>,
    pub transmitters: Vec<Transmitter>,
    pub settings: SceneSettings,
}

impl Deref for Scene {
    type Target = SceneSettings;
    fn deref(&self) -> &Self::Target {
        &self.settings
    }
}

impl DerefMut for Scene {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.settings
    }
}
