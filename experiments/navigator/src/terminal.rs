use std::collections::HashMap;

use terminal::{
    command::{Command, CommandResult},
    file_system::{Directory, File, FileSystemNode, TerminalFile},
};

#[derive(Debug, Clone, PartialEq)]
pub enum NavigatorFile {
    Terminal(TerminalFile),
}

impl File for NavigatorFile {}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct NavigatorDirectory {
    children: HashMap<String, FileSystemNode<NavigatorFile, NavigatorDirectory>>,
}

impl Directory for NavigatorDirectory {
    type Node = FileSystemNode<NavigatorFile, NavigatorDirectory>;
    fn child(&self, name: &str) -> Option<&Self::Node> {
        self.children.get(name)
    }

    fn child_mut(&mut self, name: &str) -> Option<&mut Self::Node> {
        self.children.get_mut(name)
    }

    fn children(&self) -> Vec<String> {
        self.children.keys().cloned().collect()
    }
}

pub struct TestCommand;
impl<T, D> Command<T, D> for TestCommand {
    fn name() -> &'static str {
        "test"
    }

    fn execute(
        _terminal: &mut terminal::Terminal<T, D>,
        _args: &[&str],
    ) -> terminal::command::CommandResult {
        CommandResult::Handled
    }
}
