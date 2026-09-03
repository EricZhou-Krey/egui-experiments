use crate::{
    scene::SceneObjectKey,
    settings::{logic_sheet::generate_sample_emitter_sound, SoundSettings},
};
use kira::{AudioManager, AudioManagerSettings, DefaultBackend};

pub struct SoundState {
    audio_manager: AudioManager,
    pub settings: SoundSettings,
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
    pub fn play_sound(&mut self, receiver_key: Option<SceneObjectKey>) {
        if let Some(_receiver) = receiver_key {
            todo!();
        } else {
            let _ = self.audio_manager.play(generate_sample_emitter_sound());
        }
    }
}
