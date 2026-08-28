use std::ops::{Deref, DerefMut};

use terminal::{
    file_system::{Directory, File, FileSystemNode, TerminalDirectory, TerminalFile},
    Terminal,
};

use crate::{scene::Scene, scene_object::SceneObject, state::TTSState};

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

impl File for SceneObject {}
impl Directory for Scene {
    type Node = FileSystemNode<SceneObject, Scene>;
    fn children(&self) -> &std::collections::HashMap<String, Self::Node> {
        todo!()
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
