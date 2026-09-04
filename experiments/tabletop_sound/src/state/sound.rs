use crate::{
    scene::{scene_object::Receiver, scene_viewer::SceneViewer},
    settings::SoundSettings,
};
use glam::Vec2;
use kira::{
    sound::static_sound::StaticSoundData, AudioManager, AudioManagerSettings, DefaultBackend,
};

pub struct SoundState {
    audio_manager: AudioManager,
    pub settings: SoundSettings,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SoundFilter {
    pub volume: f64,
    pub delay_seconds: f64,
    pub low_pass_cutoff_hz: f64,
    pub panning: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PointSound {
    pub apparent_position: Vec2,
    pub sound_data: StaticSoundData,
    pub filter: SoundFilter,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SoundDescriptor {
    pub paths: Vec<PointSound>,
}

impl Default for SoundState {
    fn default() -> Self {
        Self {
            audio_manager: AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
                .unwrap(),
            settings: SoundSettings::default(),
        }
    }
}

impl SoundState {
    pub fn generate_scene_descriptor(
        &self,
        receiver_position: Vec2,
        scene_viewer: SceneViewer,
    ) -> SoundDescriptor {
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
