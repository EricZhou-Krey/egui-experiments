use crate::scene_object::SceneObject;
use kira::{AudioManager, AudioManagerSettings, DefaultBackend};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

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
    pub objects: Vec<Rc<RefCell<SceneObject>>>,
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
