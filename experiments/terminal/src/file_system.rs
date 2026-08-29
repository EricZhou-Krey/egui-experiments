use std::collections::HashMap;

pub trait File {}

pub trait Directory {
    type Node;
    fn get_child(&self, name: &str) -> Option<&Self::Node>;
    fn get_child_mut(&mut self, name: &str) -> Option<&mut Self::Node>;
    fn list_children(&self) -> Vec<String>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileSystemNode<F, D> {
    File(F),
    Directory(D),
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TextFile {
    pub content: String,
}
impl File for TextFile {}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct BinaryFile {
    pub data: Vec<u8>,
}
impl File for BinaryFile {}

#[derive(Debug, Clone, PartialEq)]
pub enum TerminalFile {
    Text(TextFile),
    BinaryFile(BinaryFile),
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TerminalDirectory {
    pub children: HashMap<String, FileSystemNode<TerminalFile, TerminalDirectory>>,
}

impl Directory for TerminalDirectory {
    type Node = FileSystemNode<TerminalFile, TerminalDirectory>;
    fn get_child(&self, name: &str) -> Option<&Self::Node> {
        self.children.get(name)
    }
    fn get_child_mut(&mut self, name: &str) -> Option<&mut Self::Node> {
        self.children.get_mut(name)
    }
    fn list_children(&self) -> Vec<String> {
        self.children.keys().cloned().collect()
    }
}
