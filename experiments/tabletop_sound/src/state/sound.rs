use std::ops::{Deref, DerefMut};

use kira::{AudioManager, AudioManagerSettings, DefaultBackend};

use crate::{logic_sheet::generate_sample_transmitter_sound, state::TTSState};

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
    pub fn play_sound(&mut self, receiver_index: Option<usize>) {
        if let Some(_receiver) = receiver_index {
            todo!();
        } else {
            let _ = self.audio_manager.play(generate_sample_transmitter_sound());
        }
    }
}
