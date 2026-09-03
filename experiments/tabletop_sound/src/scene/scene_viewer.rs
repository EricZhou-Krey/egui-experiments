use crate::{
    scene::{scene_object::SceneObject, Scene, SceneObjectKey},
    tabs::terminal::TTSTerminal,
};
use glam::Vec2;
use slotmap::SlotMap;

pub struct SceneViewer<'a> {
    pub scene: &'a Scene,
    pub terminal: &'a TTSTerminal,
}

impl<'a> SceneViewer<'a> {
    pub fn find_scene_object_index_around(
        &self,
        position: Vec2,
        radius: f32,
    ) -> Option<SceneObjectKey> {
        self.scene
            .objects
            .iter()
            .find(|(_, object)| object.shape().is_around(position, radius))
            .map(|(key, _)| key)
    }

    pub fn objects(&self) -> &SlotMap<SceneObjectKey, SceneObject> {
        &self.scene.objects
    }
}
