use kira::sound::static_sound::StaticSoundData;

use crate::{sound::SoundState, state::terminal::TTSTerminal};

pub struct SoundEditor<'a> {
    sound: &'a mut SoundState,
    terminal: &'a mut TTSTerminal,
}

impl<'a> SoundEditor<'a> {
    pub fn new(sound: &'a mut SoundState, terminal: &'a mut TTSTerminal) -> Self {
        Self { sound, terminal }
    }
}
