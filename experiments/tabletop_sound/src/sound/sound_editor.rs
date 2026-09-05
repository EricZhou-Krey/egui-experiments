use kira::sound::static_sound::StaticSoundData;

use crate::{sound::SoundState, state::terminal::TTSTerminalState};

pub struct SoundEditor<'a> {
    sound: &'a mut SoundState,
    terminal: &'a mut TTSTerminalState,
}

impl<'a> SoundEditor<'a> {
    pub fn new(sound: &'a mut SoundState, terminal: &'a mut TTSTerminalState) -> Self {
        Self { sound, terminal }
    }
}
