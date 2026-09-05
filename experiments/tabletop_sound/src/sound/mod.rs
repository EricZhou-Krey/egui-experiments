pub mod ray;
pub mod sound_editor;
pub mod sound_viewer;

/*

Raytrace Planning
INPUTS
    - Scene Viewer - quick access to collsion points, and normals -should calculate normals dynamically from scene viewer
        - Receiver position to emit rays from, emitters to consume rays or capture sounds

INTERNALS
    - Ray defintion, using lifetimes (~ distance travelled) to determine the contribution level

OUTPUTS
    - Virtual position for each emitter sound, with corresponding filters applied depending
        - Each emitter has a direct, reflected and ambient contribution
            - Direct is located onto of emitters original position and low-pass filter applied if blocked with transmittence contribution
            - Reflected is virtually located at the point of greatest ray contribution from the emitter (loudest)
            - Ambient is collection of residual rays that have little contribution
    - Construct the sound by graphing and moving the 3 virtual emitter corresponding to each emitter where short rays have
    high contribution and far rays come in later to contribute adding a delay for which the sound is simulated

    - Generate a sound environment, and update when apprioate, when things are added or removed and etc
*/
use std::{collections::HashMap, f32::consts::PI};

use crate::{
    scene::{
        scene_object::{Receiver, SceneObject},
        scene_viewer::SceneViewer,
        SceneObjectKey,
    },
    settings::{logic_sheet::N_RAYS, SoundSettings},
    sound::ray::SoundRay,
};
use glam::Vec2;
use kira::{
    sound::static_sound::StaticSoundData, AudioManager, AudioManagerSettings, DefaultBackend,
};
use slotmap::{new_key_type, SlotMap};

new_key_type! { pub struct SoundKey; }

pub struct SoundState {
    audio_manager: AudioManager,
    pub sounds: SlotMap<SoundKey, StaticSoundData>,
    pub settings: SoundSettings,
}

impl Default for SoundState {
    fn default() -> Self {
        Self {
            audio_manager: AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
                .unwrap(),
            sounds: SlotMap::with_key(),
            settings: SoundSettings::default(),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct SoundFilter {
    pub volume: f64,
    pub delay_seconds: f64,
    pub low_pass_cutoff_hz: f64,
    pub panning: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PointSound {
    pub apparent_position: Vec2,
    pub sound_key: SoundKey,
    pub filter: SoundFilter,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SoundDescriptor {
    pub paths: Vec<PointSound>,
}

impl SoundState {
    pub fn generate_scene_descriptor(
        &self,
        receiver_position: Vec2,
        scene_viewer: SceneViewer,
    ) -> SoundDescriptor {
        let sound_rays: Vec<SoundRay> = (0..N_RAYS)
            .map(|i| SoundRay {
                direction: Vec2::from_angle(2.0 * PI * (i as f32 / N_RAYS as f32)),
                distance_travelled: 0.0,
            })
            .collect();

        struct EmitterPointSound {
            pub direct: PointSound,
            pub reflected: PointSound,
        }
        // Making too many copies of the sound file, should only store the sound data once and
        // reference it from the file system probably, need to implement that as cloning is
        // impratcical before moving on
        let emitter_point_sounds: HashMap<SceneObjectKey, EmitterPointSound> = scene_viewer
            .emitter_keys()
            .iter()
            .filter_map(|key| {
                if let Some(SceneObject::Emitter(emitter)) = scene_viewer.object(*key) && let Some(sound_key) = emitter.sound_key {
                    Some((
                        *key,
                        EmitterPointSound {
                            direct: PointSound {
                                apparent_position: emitter.shape.center(),
                                sound_key,
                                filter: SoundFilter::default(),
                            },
                            reflected: PointSound {
                                apparent_position: Vec2::ZERO, // TODO
                                sound_key,
                                filter: SoundFilter::default(),
                            },
                        },
                    ))
                } else {
                    None
                }
            })
            .collect();

        for ray in sound_rays {
            todo!()
        }

        // Pseudo-code for raytracing:
        // 3. For each successful ray path (direct, penetration, bounce):
        //    - Calculate total distance -> map to `delay_seconds` & base `volume`
        //    - Count walls penetrated -> lower `low_pass_cutoff_hz` and `volume`
        //    - Calculate arrival vector -> map to `panning`
        // 4. Push as a new `PointSound` to SoundDescriptor

        todo!()
    }

    pub fn play(&mut self, receiver: &Receiver, scene_viewer: SceneViewer) {
        // for point_sound in descriptor.paths {
        //     // To implement this in Kira:
        //     // 1. Create a `Track` with a LowPassBuilder effect for the `low_pass_cutoff_hz`.
        //     // 2. Play the `StaticSoundData` on that track.
        //     // 3. Apply `panning` and `volume` via `StaticSoundSettings`.
        //     // 4. (Optional) handle `delay_seconds` via Kira clocks or your own game loop timers.
        // }
        todo!()
    }
}
