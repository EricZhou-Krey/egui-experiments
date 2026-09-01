use std::ops::{Deref, DerefMut};

use kira::{AudioManager, AudioManagerSettings, DefaultBackend};

use crate::{logic_sheet::generate_sample_transmitter_sound, state::TTSState};

#[derive(Debug, Clone, PartialEq)]
pub struct SoundSettings {
    volume: f32,
}

impl Default for SoundSettings {
    fn default() -> Self {
        Self { volume: 1.0 }
    }
}

pub struct SoundState {
    audio_manager: AudioManager,
    pub selected_receiver: Option<usize>,
    pub settings: SoundSettings,
}

impl Deref for SoundState {
    type Target = SoundSettings;
    fn deref(&self) -> &Self::Target {
        &self.settings
    }
}

impl DerefMut for SoundState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.settings
    }
}

impl Default for SoundState {
    fn default() -> Self {
        Self {
            audio_manager: AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
                .unwrap(),
            selected_receiver: None,
            settings: SoundSettings::default(),
        }
    }
}

impl SoundState {
    pub fn play_sound(&mut self) {
        if let Some(_receiver) = &self.selected_receiver {
            todo!();
        } else {
            let _ = self.audio_manager.play(generate_sample_transmitter_sound());
        }
    }
}

pub fn soundview_title(_state: &mut TTSState) -> egui::WidgetText {
    "SoundView".into()
}

pub fn soundview_ui(_state: &mut TTSState, ui: &mut egui::Ui) {
    ui.centered_and_justified(|ui| ui.heading("SoundView"));
}
