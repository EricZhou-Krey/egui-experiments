use kira::{
    sound::static_sound::StaticSoundData, AudioManager, AudioManagerSettings, DefaultBackend,
};

use crate::style::{FaceStyle, PointStyle};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq)]
pub struct Wall {
    pub verticies: Vec<[f32; 2]>,
    pub face_style: FaceStyle,
    pub vertex_style: PointStyle,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Receiver {
    pub position: [f32; 2],
    pub style: PointStyle,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Transmitter {
    pub position: [f32; 2],
    pub sound_data: StaticSoundData,
    pub style: PointStyle,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SceneObject {
    Wall(Box<Wall>),
    Receiver(Box<Receiver>),
    Transmitter(Box<Transmitter>),
}

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
    objects: Vec<SceneObject>,
    audio_manager: AudioManager,
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
    pub fn find_object_index_around(&self, position: [f32; 2], radius: f32) -> Option<usize> {
        self.objects
            .iter()
            .rposition(|object: &SceneObject| -> bool {
                match object {
                    SceneObject::Transmitter(transmitter) => {
                        let dx: f32 = position[0] - transmitter.position[0];
                        let dy: f32 = position[1] - transmitter.position[1];
                        (dx * dx + dy * dy) <= (radius * radius)
                    }
                    SceneObject::Receiver(receiver) => {
                        let dx: f32 = position[0] - receiver.position[0];
                        let dy: f32 = position[1] - receiver.position[1];
                        (dx * dx + dy * dy) <= (radius * radius)
                    }
                    SceneObject::Wall(wall) => {
                        if wall.verticies.len() < 3 {
                            return false;
                        }
                        let mut has_negative: bool = false;
                        let mut has_positive: bool = false;
                        for i in 0..wall.verticies.len() {
                            let p1: [f32; 2] = wall.verticies[i];
                            let p2: [f32; 2] = wall.verticies[(i + 1) % wall.verticies.len()];
                            let cross: f32 = (position[0] - p1[0]) * (p2[1] - p1[1])
                                - (position[1] - p1[1]) * (p2[0] - p1[0]);
                            if cross < 0.0 {
                                has_negative = true;
                            } else if cross > 0.0 {
                                has_positive = true;
                            }
                            if has_negative && has_positive {
                                return false;
                            }
                        }
                        true
                    }
                }
            })
    }

    pub fn objects(&self) -> &Vec<SceneObject> {
        &self.objects
    }

    pub fn add_object(&mut self, object: SceneObject) {
        // TODO: TEMP
        if let SceneObject::Transmitter(transmitter) = &object {
            self.audio_manager
                .play(transmitter.sound_data.clone())
                .unwrap();
        }

        self.objects.push(object);
    }

    pub fn move_object(&mut self, index: usize, delta: [f32; 2]) -> bool {
        if let Some(object) = self.objects.get_mut(index) {
            match object {
                SceneObject::Wall(wall) => {
                    for vertex in &mut wall.verticies {
                        vertex[0] += delta[0];
                        vertex[1] += delta[1];
                    }
                }
                SceneObject::Receiver(receiver) => {
                    receiver.position[0] += delta[0];
                    receiver.position[1] += delta[1];
                }
                SceneObject::Transmitter(transmitter) => {
                    transmitter.position[0] += delta[0];
                    transmitter.position[1] += delta[1];
                }
            }
            true
        } else {
            false
        }
    }
    pub fn remove_object(&mut self, index: usize) {
        self.objects.remove(index);
    }
}
