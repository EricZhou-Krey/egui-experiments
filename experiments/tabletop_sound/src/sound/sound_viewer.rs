use crate::{sound::SoundState, state::terminal::TTSTerminal};

pub struct SoundViewer<'a> {
    sound: &'a SoundState,
    terminal: &'a TTSTerminal,
}

impl<'a> SoundViewer<'a> {
    pub fn new(sound: &'a SoundState, terminal: &'a TTSTerminal) -> Self {
        Self { sound, terminal }
    }
}
