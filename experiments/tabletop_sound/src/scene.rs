use crate::scene_object::SceneObject;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SceneSettings {}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Scene {
    pub objects: Vec<Rc<RefCell<SceneObject>>>,
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
