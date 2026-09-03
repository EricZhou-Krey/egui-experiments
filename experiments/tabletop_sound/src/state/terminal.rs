use std::{
    cell::RefCell,
    collections::HashMap,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use terminal::{
    Terminal, command::{Command, CommandResult}, file_system::{Directory, File, FileSystemNode, TerminalDirectory, TerminalFile}
};

use crate::{scene::Scene, scene_object::SceneObject, state::TTSState};

#[derive(Debug, Clone, PartialEq)]
pub enum TTSFile {
    Terminal(TerminalFile),
    SceneObject(Rc<RefCell<SceneObject>>),
}

impl Default for TTSFile {
    fn default() -> Self {
        Self::Terminal(TerminalFile::default())
    }
}

impl From<TerminalFile> for TTSFile {
    fn from(value: TerminalFile) -> Self {
        Self::Terminal(value)
    }
}

impl TryFrom<TTSFile> for TerminalFile {
    type Error = &'static str;
    fn try_from(value: TTSFile) -> Result<TerminalFile, Self::Error> {
        match value {
            TTSFile::Terminal(t) => Ok(t),
            TTSFile::SceneObject(..) => Err("Cannot convert SceneObject to TerminalFile"),
        }
    }
}

impl File for TTSFile {}

#[derive(Clone)]
pub enum TTSDirectory {
    Terminal(HashMap<String, FileSystemNode<TTSFile, TTSDirectory>>),
    Scene {
        scene: Rc<RefCell<Scene>>,
        nodes: HashMap<String, FileSystemNode<TTSFile, TTSDirectory>>,
    },
}

impl Default for TTSDirectory {
    fn default() -> Self {
        Self::Terminal(HashMap::new())
    }
}

impl From<TerminalDirectory> for TTSDirectory {
    fn from(value: TerminalDirectory) -> Self {
        let mut children = HashMap::new();
        for (k, v) in value.children {
            let node = match v {
                FileSystemNode::File(f) => FileSystemNode::File(TTSFile::from(f)),
                FileSystemNode::Directory(d) => FileSystemNode::Directory(TTSDirectory::from(d)),
            };
            children.insert(k, node);
        }
        Self::Terminal(children)
    }
}

impl TryFrom<TTSDirectory> for TerminalDirectory {
    type Error = &'static str;
    fn try_from(value: TTSDirectory) -> Result<Self, Self::Error> {
        match value {
            TTSDirectory::Terminal(children) => {
                let mut term_children = HashMap::new();
                for (k, v) in children {
                    let node = match v {
                        FileSystemNode::File(f) => FileSystemNode::File(TerminalFile::try_from(f)?),
                        FileSystemNode::Directory(d) => {
                            FileSystemNode::Directory(TerminalDirectory::try_from(d)?)
                        }
                    };
                    term_children.insert(k, node);
                }
                Ok(TerminalDirectory {
                    children: term_children,
                })
            }
            TTSDirectory::Scene { .. } => Err("Cannot convert Scene to TerminalDirectory"),
        }
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

struct TTSCatCommand;
impl<D> Command<TTSFile, D> for TTSCatCommand
where
    D: Directory<Node = FileSystemNode<TTSFile, D>>
{
    fn name() -> &'static str { "cat" }
    fn execute(terminal: &mut Terminal<TTSFile, D>, args: &[&str]) -> CommandResult {
        if let Some(target_file) = args.first() {
            let mut file_path: Vec<String> = terminal.current_directory.clone();
            file_path.push(target_file.to_string());

            if let Some(FileSystemNode::File(file)) = terminal.get_node(&file_path) {
                match file {
                    TTSFile::Terminal(terminal_file) => {
                        match terminal_file {
                            TerminalFile::Text(text_file) => {
                                terminal.history.push(text_file.content.clone());
                            }
                            TerminalFile::Binary(_) => {
                                terminal.history.push(format!("cat: {}: cannot display binary file", target_file));
                            }
                        }
                    },
                    TTSFile::SceneObject(object_file) => {
                        terminal.history.push(format!("{:?}", object_file.borrow()));
                    }
                }
            } else {
                terminal.history.push(format!("cat: {}: No such file", target_file));
            }
        } else {
            terminal.history.push("cat: missing file operand".to_string());
        }
        CommandResult::Handled
    }
}

#[derive(Clone)]
pub struct TTSTerminalState {
    terminal: Terminal<TTSFile, TTSDirectory>,
}

impl TTSTerminalState {
    pub fn new(scene: Rc<RefCell<Scene>>) -> Self {
        let base_terminal: Terminal<TerminalFile, TerminalDirectory> =
            terminal::Terminal::<TerminalFile, TerminalDirectory>::default();

        let mut tts_file_system: FileSystemNode<TTSFile, TTSDirectory> =
            match base_terminal.file_system {
                FileSystemNode::Directory(d) => FileSystemNode::Directory(TTSDirectory::from(d)),
                FileSystemNode::File(f) => FileSystemNode::File(TTSFile::from(f)),
            };

        if let FileSystemNode::Directory(TTSDirectory::Terminal(ref mut root_children)) =
            tts_file_system
        {
            root_children.insert(
                "scene".to_string(),
                FileSystemNode::Directory(TTSDirectory::Scene {
                    scene,
                    nodes: HashMap::new(),
                }),
            );
        }

        let mut terminal: Terminal<TTSFile, TTSDirectory> = Terminal::<TTSFile, TTSDirectory> {
            history: base_terminal.history,
            input: base_terminal.input,
            current_directory: base_terminal.current_directory,
            file_system: tts_file_system,
            style: base_terminal.style,
            commands: std::collections::HashMap::new(),
        };

        terminal.register_command::<terminal::command::ClearCommand>();
        terminal.register_command::<terminal::command::PwdCommand>();
        terminal.register_command::<terminal::command::LsCommand>();
        terminal.register_command::<terminal::command::CdCommand>();
        terminal.register_command::<TTSCatCommand>();
        terminal.register_command::<terminal::command::NeofetchCommand>();
        terminal.register_command::<terminal::command::HelpCommand>();

        Self { terminal }
    }

    pub fn add_scene_object(&mut self, index: usize, object: Rc<RefCell<SceneObject>>) {
        if let FileSystemNode::Directory(TTSDirectory::Terminal(children)) = &mut self.terminal.file_system &&
            let Some(FileSystemNode::Directory(TTSDirectory::Scene { nodes, .. })) = children.get_mut("scene") {
            
            let object_type = object.borrow().type_name(); 
            
            let filename: String = format!("{}_{}.obj", object_type, index);
            nodes.insert(filename, FileSystemNode::File(TTSFile::SceneObject(object)));
        }
    }

    pub fn remove_scene_object(&mut self, index: usize, new_total: usize) {
        if let FileSystemNode::Directory(TTSDirectory::Terminal(children)) = &mut self.terminal.file_system && 
             let Some(FileSystemNode::Directory(TTSDirectory::Scene { nodes, .. })) = children.get_mut("scene") {
             
             let target_suffix = format!("_{}.obj", index);
             let target_key = nodes.keys().find(|k| k.ends_with(&target_suffix)).cloned();
             
             if let Some(key) = target_key {
                 nodes.remove(&key);
             }

             for i in (index + 1)..=new_total {
                 let old_suffix = format!("_{}.obj", i);
                 let old_key = nodes.keys().find(|k| k.ends_with(&old_suffix)).cloned();
                 
                 if let Some(key) = old_key && let Some(node) = nodes.remove(&key) {
                     let new_suffix = format!("_{}.obj", i - 1);
                     let new_filename = key.replace(&old_suffix, &new_suffix);
                     nodes.insert(new_filename, node);
                 }
             }
        }
    }
}
