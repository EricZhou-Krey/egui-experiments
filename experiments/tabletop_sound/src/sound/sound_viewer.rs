use crate::{sound::SoundState, state::terminal::TTSTerminalState};

pub struct SoundViewer<'a> {
    sound: &'a SoundState,
    terminal: &'a TTSTerminalState,
}

impl<'a> SoundViewer<'a> {
    pub fn new(sound: &'a SoundState, terminal: &'a TTSTerminalState) -> Self {
        Self { sound, terminal }
    }
}
