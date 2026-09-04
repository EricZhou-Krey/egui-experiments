pub mod scene_editor;
pub mod scene_object;
pub mod scene_viewer;
use std::collections::HashSet;

use crate::scene::scene_object::SceneObject;
use crate::settings::SceneSettings;
use glam::Vec2;
use rstar::{RTree, RTreeObject, AABB};
use slotmap::{new_key_type, SlotMap};

new_key_type! { pub struct SceneObjectKey; }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpatialNode {
    pub key: SceneObjectKey,
    pub min: Vec2,
    pub max: Vec2,
}

impl RTreeObject for SpatialNode {
    type Envelope = AABB<[f32; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(self.min.into(), self.max.into())
    }
}

#[derive(Debug)]
pub struct Scene {
    pub objects: SlotMap<SceneObjectKey, SceneObject>,
    pub receiver_keys: HashSet<SceneObjectKey>,
    pub emitter_keys: HashSet<SceneObjectKey>,
    pub wall_quadtree: RTree<SpatialNode>,
    pub settings: SceneSettings,
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: SlotMap::with_key(),
            wall_quadtree: RTree::new(),
            ..Default::default()
        }
    }
}
