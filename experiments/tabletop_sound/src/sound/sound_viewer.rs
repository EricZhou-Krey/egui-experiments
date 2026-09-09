use crate::{sound::SoundState, state::terminal::TTSTerminal};

pub struct SoundViewer<'a> {
    _sound: &'a SoundState,
    _terminal: &'a TTSTerminal,
}

impl<'a> SoundViewer<'a> {
    pub fn new(_sound: &'a SoundState, _terminal: &'a TTSTerminal) -> Self {
        Self { _sound, _terminal }
    }
}
