use crate::{
    scene::{scene_object::SceneObject, Scene, SceneObjectKey, SpatialNode},
    state::terminal::TTSTerminalState,
};
use glam::Vec2;

pub struct SceneEditor<'a> {
    pub scene: &'a mut Scene,
    pub terminal: &'a mut TTSTerminalState,
}

impl<'a> SceneEditor<'a> {
    pub fn translate_scene_object(&mut self, key: SceneObjectKey, delta: Vec2) {
        if let Some(object) = self.scene.objects.get_mut(key) {
            let (old_min, old_max): (Vec2, Vec2) = object.shape().logical_bounds();
            let old_node = SpatialNode {
                key,
                min: old_min,
                max: old_max,
            };
            self.scene.quadtree.remove(&old_node);

            object.mut_shape().translate(delta);
            let (new_min, new_max): (Vec2, Vec2) = object.shape().logical_bounds();
            let new_node = SpatialNode {
                key,
                min: new_min,
                max: new_max,
            };

            self.scene.quadtree.insert(new_node);
        }
    }

    pub fn add_scene_object(&mut self, object: SceneObject) -> SceneObjectKey {
        let (min, max): (Vec2, Vec2) = object.shape().logical_bounds();
        let key: SceneObjectKey = self.scene.objects.insert(object);
        let node: SpatialNode = SpatialNode { key, min, max };
        self.scene.quadtree.insert(node);
        self.terminal.register_object(key);
        key
    }

    pub fn remove_scene_object(&mut self, key: SceneObjectKey) -> Option<SceneObject> {
        if let Some(object) = self.scene.objects.remove(key) {
            let (min, max): (Vec2, Vec2) = object.shape().logical_bounds();
            let node: SpatialNode = SpatialNode { key, min, max };
            self.scene.quadtree.remove(&node);
            self.terminal.deregister_object(key);
            Some(object)
        } else {
            None
        }
    }
}
