use crate::{
    scene::{scene_object::SceneObject, Scene, SceneObjectKey, SpatialNode},
    state::{sound::SoundState, terminal::TTSTerminalState},
};
use glam::Vec2;

pub struct SceneEditor<'a> {
    pub scene: &'a mut Scene,
    pub terminal: &'a mut TTSTerminalState,
    pub sound: &'a mut SoundState,
}

impl<'a> SceneEditor<'a> {
    pub fn modify_object<F>(&mut self, key: SceneObjectKey, mut modifier: F)
    where
        F: FnMut(&mut SceneObject),
    {
        if let Some(object) = self.scene.objects.get_mut(key) {
            if !matches!(object, SceneObject::Wall(..)) {
                modifier(object);
                return;
            }

            let (old_min, old_max): (Vec2, Vec2) = object.shape().logical_bounds();
            modifier(object);
            let (new_min, new_max): (Vec2, Vec2) = object.shape().logical_bounds();

            if old_min != new_min || old_max != new_max {
                let old_node = SpatialNode {
                    key,
                    min: old_min,
                    max: old_max,
                };
                self.scene.wall_quadtree.remove(&old_node);

                let (new_min, new_max): (Vec2, Vec2) = object.shape().logical_bounds();
                let new_node = SpatialNode {
                    key,
                    min: new_min,
                    max: new_max,
                };
                self.scene.wall_quadtree.insert(new_node);
            }
        }
    }

    pub fn add_object(&mut self, object: SceneObject) -> SceneObjectKey {
        let key: SceneObjectKey = self.scene.objects.insert(object);
        self.terminal.register_object(self.scene, key);

        let object: &SceneObject = self.scene.objects.get(key).unwrap();
        match object {
            SceneObject::Wall(..) => {
                let (min, max): (Vec2, Vec2) = object.shape().logical_bounds();
                let node: SpatialNode = SpatialNode { key, min, max };
                self.scene.wall_quadtree.insert(node);
            }
            SceneObject::Emitter(..) => {
                self.scene.emitter_keys.insert(key);
            }
            SceneObject::Receiver(..) => {
                self.scene.receiver_keys.insert(key);
            }
        }

        key
    }

    pub fn remove_object(&mut self, key: SceneObjectKey) -> Option<SceneObject> {
        if let Some(object) = self.scene.objects.remove(key) {
            self.terminal.deregister_object(self.scene, key);

            match object {
                SceneObject::Wall(..) => {
                    let (min, max): (Vec2, Vec2) = object.shape().logical_bounds();
                    let node: SpatialNode = SpatialNode { key, min, max };
                    self.scene.wall_quadtree.remove(&node);
                }
                SceneObject::Emitter(..) => {
                    self.scene.emitter_keys.remove(&key);
                }
                SceneObject::Receiver(..) => {
                    self.scene.receiver_keys.remove(&key);
                }
            }

            Some(object)
        } else {
            None
        }
    }
}
