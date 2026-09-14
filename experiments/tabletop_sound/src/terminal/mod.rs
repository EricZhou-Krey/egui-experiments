pub mod command;
pub mod file_system;

use terminal::{
    Terminal, app::AppTerminal, file_system::FileSystemNode
};

use crate::{
    scene::{Scene, SceneObjectKey, scene_object::SceneObject}, state::TTSState, terminal::{
        command::TTSInfoCommand, file_system::{TTSDirectory, TTSFile}
    }
};
use std::collections::HashMap;

pub type TTSTerminal = AppTerminal<TTSFile, TTSDirectory, TTSState>;

pub fn create_tts_terminal() -> TTSTerminal {
    let mut tts_file_system: FileSystemNode<TTSFile, TTSDirectory> = FileSystemNode::Directory(TTSDirectory::default());

    if let FileSystemNode::Directory(TTSDirectory::Terminal(ref mut root_children)) = tts_file_system {
        root_children.insert(
            "scene".to_string(),
            FileSystemNode::Directory(TTSDirectory::Scene {
                nodes: HashMap::new(),
            }),
        );
    }

    let base = Terminal::<TTSFile, TTSDirectory>::new(tts_file_system);
    let mut terminal = TTSTerminal::new(base);

    terminal.register_app_command::<TTSInfoCommand>();

    terminal
}

pub trait TTSTerminalExt {
    fn register_object(&mut self, scene: &mut Scene, object_key: SceneObjectKey);
    fn deregister_object(&mut self, scene: &mut Scene, object_key: SceneObjectKey);
}

impl TTSTerminalExt for TTSTerminal {
    fn register_object(&mut self, scene: &mut Scene, object_key: SceneObjectKey) {
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

    fn deregister_object(&mut self, _scene: &mut Scene, object_key: SceneObjectKey) {
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
}
