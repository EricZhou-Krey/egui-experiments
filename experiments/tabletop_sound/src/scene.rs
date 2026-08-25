use crate::scene_object::SceneObject;
use glam::Vec2;
use kira::{AudioManager, AudioManagerSettings, DefaultBackend};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq)]
pub struct SceneAudioSettings {
    pub volume: f32,
}

impl Default for SceneAudioSettings {
    fn default() -> Self {
        Self { volume: 1.0 }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SceneSettings {
    pub audio: SceneAudioSettings,
}

pub struct Scene {
    pub objects: Vec<SceneObject>,
    pub audio_manager: AudioManager,
    pub settings: SceneSettings,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            objects: Vec::default(),
            audio_manager: AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
                .unwrap(),
            settings: SceneSettings::default(),
        }
    }
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

impl Scene {
    pub fn find_object_index_around(&self, position: Vec2, radius: f32) -> Option<usize> {
        self.objects
            .iter()
            .rposition(|object: &SceneObject| -> bool {
                object.shape().is_around(position, radius)
            })
    }
}
