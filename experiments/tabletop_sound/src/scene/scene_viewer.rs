use crate::{
    scene::{scene_object::SceneObject, Scene, SceneObjectKey, SpatialNode},
    state::{sound::SoundState, terminal::TTSTerminalState},
};
use glam::Vec2;
use slotmap::basic::{Iter, Values};
use std::collections::HashSet;

pub struct SceneViewer<'a> {
    pub scene: &'a Scene,
    pub terminal: &'a TTSTerminalState,
    pub sound: &'a SoundState,
}

impl<'a> SceneViewer<'a> {
    pub fn key_object_around(
        &self,
        position: Vec2,
        radius: f32,
    ) -> Option<(SceneObjectKey, &SceneObject)> {
        self.scene
            .objects
            .iter()
            .find(|(_, object)| object.shape().is_around(position, radius))
    }

    pub fn object(&self, key: SceneObjectKey) -> Option<&SceneObject> {
        self.scene.objects.get(key)
    }

    pub fn emitter_keys(&self) -> &HashSet<SceneObjectKey> {
        &self.scene.emitter_keys
    }

    pub fn receiver_keys(&self) -> &HashSet<SceneObjectKey> {
        &self.scene.receiver_keys
    }

    pub fn wall_keys(&self) -> HashSet<SceneObjectKey> {
        self.scene
            .wall_quadtree
            .iter()
            .map(|spatial_node: &SpatialNode| spatial_node.key)
            .collect()
    }

    pub fn objects(&self) -> Values<'_, SceneObjectKey, SceneObject> {
        self.scene.objects.values()
    }

    pub fn key_objects(&self) -> Iter<'_, SceneObjectKey, SceneObject> {
        self.scene.objects.iter()
    }
}
