use crate::{
    scene::{scene_object::SceneObject, Scene, SceneObjectKey},
    state::terminal::TTSTerminalState,
};
use glam::Vec2;
use slotmap::basic::Values;

pub struct SceneViewer<'a> {
    pub scene: &'a Scene,
    pub terminal: &'a TTSTerminalState,
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

    pub fn objects(&self) -> Values<'_, SceneObjectKey, SceneObject> {
        self.scene.objects.values()
    }
}
