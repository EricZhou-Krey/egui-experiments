use std::ops::{Deref, DerefMut};

use terminal::{
    file_system::{TerminalDirectory, TerminalFile},
    Terminal,
};

use crate::state::TTSState;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TTSTerminal {
    internal_terminal: Terminal<TerminalFile, TerminalDirectory>,
}

impl Deref for TTSTerminal {
    type Target = Terminal<TerminalFile, TerminalDirectory>;
    fn deref(&self) -> &Self::Target {
        &self.internal_terminal
    }
}

impl DerefMut for TTSTerminal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.internal_terminal
    }
}

pub fn terminal_title(_state: &mut TTSState) -> egui::WidgetText {
    "Terminal".into()
}

pub fn terminal_ui(state: &mut TTSState, ui: &mut egui::Ui) {
    state.terminal.ui(ui);
}
