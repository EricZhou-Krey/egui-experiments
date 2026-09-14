use std::collections::HashMap;

use terminal::file_system::{Directory, File, FileSystemNode, TerminalFile};

use crate::scene::SceneObjectKey;

#[derive(Debug, Clone, PartialEq)]
pub enum TTSFile {
    Terminal(TerminalFile),
    SceneObject(SceneObjectKey),
}

impl Default for TTSFile {
    fn default() -> Self {
        Self::Terminal(TerminalFile::default())
    }
}

impl File for TTSFile {}

#[derive(Clone)]
pub enum TTSDirectory {
    Terminal(HashMap<String, FileSystemNode<TTSFile, TTSDirectory>>),
    Scene {
        nodes: HashMap<String, FileSystemNode<TTSFile, TTSDirectory>>,
    },
}

impl Default for TTSDirectory {
    fn default() -> Self {
        Self::Terminal(HashMap::new())
    }
}

impl Directory for TTSDirectory {
    type Node = FileSystemNode<TTSFile, TTSDirectory>;

    fn child(&self, name: &str) -> Option<&Self::Node> {
        match self {
            Self::Terminal(children) => children.get(name),
            Self::Scene { nodes, .. } => nodes.get(name),
        }
    }

    fn child_mut(&mut self, name: &str) -> Option<&mut Self::Node> {
        match self {
            Self::Terminal(children) => children.get_mut(name),
            Self::Scene { nodes, .. } => nodes.get_mut(name),
        }
    }

    fn children(&self) -> Vec<String> {
        match self {
            Self::Terminal(children) => children.keys().cloned().collect(),
            Self::Scene { nodes, .. } => nodes.keys().cloned().collect(),
        }
    }
}
