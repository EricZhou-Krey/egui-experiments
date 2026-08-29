use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use egui::Scene;
use terminal::{
    file_system::{Directory, File, FileSystemNode, TerminalDirectory, TerminalFile},
    Terminal,
};

use crate::{scene_object::SceneObject, state::TTSState};

pub enum TTSFile {
    Terminal(TerminalFile),
    SceneObject(Rc<RefCell<SceneObject>>),
}

impl File for TTSFile {}

pub enum TTSDirectory {
    Terminal(TerminalDirectory),
    Scene(Rc<RefCell<Scene>>),
}

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
