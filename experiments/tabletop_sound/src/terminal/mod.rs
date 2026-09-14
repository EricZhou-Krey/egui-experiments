pub mod command;
pub mod file_system;

use terminal::{Terminal, command::{Command, CommandResult}, file_system::FileSystemNode};

use crate::{scene::{Scene, SceneObjectKey, scene_object::SceneObject}, state::TTSState, terminal::{command::{TTSCatCommand, TTSCommand, TTSFn}, file_system::{TTSDirectory, TTSFile}}};
use std::collections::HashMap;


#[derive(Clone)]
pub struct TTSTerminal {
    base: Terminal<TTSFile, TTSDirectory>,
    pub commands: HashMap<String, fn(&mut TTSState, &[String])>,

}

impl Default for TTSTerminal {
    fn default() -> Self {
        let mut tts_file_system: FileSystemNode<TTSFile, TTSDirectory> = FileSystemNode::Directory(TTSDirectory::default());

        if let FileSystemNode::Directory(TTSDirectory::Terminal(ref mut root_children)) =
            tts_file_system
        {
            root_children.insert(
                "scene".to_string(),
                FileSystemNode::Directory(TTSDirectory::Scene {
                    nodes: HashMap::new(),
                }),
            );
        }

        let base: Terminal<TTSFile, TTSDirectory> = Terminal::<TTSFile, TTSDirectory>::new(tts_file_system);
        let mut terminal = Self {
            base,
            commands: HashMap::new(),
        };

        terminal.register_command::<TTSCatCommand>();

        terminal
    }
}



impl TTSTerminal {
    pub fn register_object(&mut self, scene: &mut Scene, object_key: SceneObjectKey) {
        if let FileSystemNode::Directory(TTSDirectory::Terminal(children)) = &mut self.base.file_system &&
        let Some(FileSystemNode::Directory(TTSDirectory::Scene { nodes, .. })) = children.get_mut("scene") {
            let object_type: &'static str = match scene.objects.get(object_key) {
                Some(SceneObject::Wall(..)) => "wall",
                Some(SceneObject::Emitter(..)) => "emitter",
                Some(SceneObject::Receiver(..)) => "receiver",
                None => "?",
            };

            let filename: String = format!("{}_(ID: {:?}).obj", object_type, object_key);
            nodes.insert(
                filename,
                FileSystemNode::File(TTSFile::SceneObject(object_key)),
            );
        }
    }

    pub fn deregister_object(&mut self, _scene: &mut Scene, object_key: SceneObjectKey) {
        if let FileSystemNode::Directory(TTSDirectory::Terminal(children)) = &mut self.base.file_system && 
        let Some(FileSystemNode::Directory(TTSDirectory::Scene { nodes, .. })) = children.get_mut("scene") {
            let target_filename = nodes.iter().find_map(|(name, node)| {
                if let FileSystemNode::File(TTSFile::SceneObject(k)) = node && *k == object_key {
                    return Some(name.clone());
                }
                None
            });

            if let Some(filename) = target_filename {
                nodes.remove(&filename);
            }
        }
    }

    pub fn register_command<C>(&mut self)
    where
        C: Command<TTSFile, TTSDirectory> + TTSCommand,
    {
        self.base.register_command::<C>();
        self.commands
            .insert(C::name().to_string(), C::execute_tts);
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<(TTSFn, Vec<String>)> {
        if let Some(CommandResult::Unhandled(cmd, args)) = self.base.ui(ui) {
            if let Some(func) = self.commands.get(&cmd).copied() {
                return Some((func, args));
            } else {
                self.base
                    .history
                    .push(format!("{}: command not found", cmd));
            }
        }
        None
    }
}

