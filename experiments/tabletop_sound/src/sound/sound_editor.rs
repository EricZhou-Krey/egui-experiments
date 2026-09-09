use crate::{sound::SoundState, state::terminal::TTSTerminal};

pub struct SoundEditor<'a> {
    _sound: &'a mut SoundState,
    _terminal: &'a mut TTSTerminal,
}

impl<'a> SoundEditor<'a> {
    pub fn new(_sound: &'a mut SoundState, _terminal: &'a mut TTSTerminal) -> Self {
        Self { _sound, _terminal }
    }
}
