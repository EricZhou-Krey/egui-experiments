use eframe::egui;
use std::collections::HashMap;

use terminal::{
    command::{ClearCommand, Command, CommandResult, HelpCommand},
    file_system::{Directory, File, FileSystemNode, TerminalFile},
    Terminal,
};

use crate::{
    layouts::Layout,
    navigator::{Graph, GraphMode, Navigator},
    settings::style_sheet::TERMINAL_STYLE,
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

type NavigatorFn = fn(&mut Navigator, &[String]);

pub trait NavigatorCommand: Command<NavigatorFile, NavigatorDirectory> {
    fn execute_navigator(navigator: &mut Navigator, args: &[String]);
}

pub struct NavigatorTerminal {
    pub base: Terminal<NavigatorFile, NavigatorDirectory>,
    pub commands: HashMap<String, fn(&mut Navigator, &[String])>,
}

impl Default for NavigatorTerminal {
    fn default() -> Self {
        Self::new()
    }
}

impl NavigatorTerminal {
    pub fn new() -> Self {
        let root = FileSystemNode::Directory(NavigatorDirectory::default());
        let mut base = Terminal::new_empty(root);
        base.register_command::<HelpCommand>();
        base.register_command::<ClearCommand>();
        base.style = TERMINAL_STYLE;

        let mut terminal = Self {
            base,
            commands: HashMap::new(),
        };

        terminal.register_command::<SetModeCommand>();
        terminal.register_command::<SetOverlayCommand>();

        terminal.base.execute_command("help");

        terminal
    }

    pub fn register_command<C>(&mut self)
    where
        C: Command<NavigatorFile, NavigatorDirectory> + NavigatorCommand,
    {
        self.base.register_command::<C>();
        self.commands
            .insert(C::name().to_string(), C::execute_navigator);
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<(NavigatorFn, Vec<String>)> {
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

pub struct SetModeCommand;

impl Command<NavigatorFile, NavigatorDirectory> for SetModeCommand {
    fn name() -> &'static str {
        "set_mode"
    }

    fn execute(
        _terminal: &mut Terminal<NavigatorFile, NavigatorDirectory>,
        args: &[&str],
    ) -> CommandResult {
        let args_owned = args.iter().map(|s| s.to_string()).collect();
        CommandResult::Unhandled(Self::name().to_string(), args_owned)
    }
}

impl NavigatorCommand for SetModeCommand {
    fn execute_navigator(navigator: &mut Navigator, args: &[String]) {
        let history: &mut Vec<String> = &mut navigator.terminal.base.history;
        if let Some(mode) = args.first() {
            match mode.as_str() {
                "boids" => {
                    navigator.graph_mode = GraphMode::Boids;
                    navigator.graph = Graph::Boids(Box::default());
                    history.push("Switched to Boids mode.".to_string());
                }
                "life" => {
                    navigator.graph_mode = GraphMode::Life;
                    navigator.graph = Graph::Life(Box::default());
                    history.push("Switched to Game of Life mode.".to_string());
                }
                "triangulation" => {
                    navigator.graph_mode = GraphMode::Triangulation;
                    navigator.graph = Graph::Triangulation(Box::default());
                    history.push("Switched to Triangulation mode.".to_string());
                }
                _ => {
                    history.push(format!("set_mode: unknown mode '{}'", mode));
                }
            }
        } else {
            history.push("Usage: set_mode [boids | life | triangulation]".to_string());
        }
    }
}

pub struct SetOverlayCommand;

impl Command<NavigatorFile, NavigatorDirectory> for SetOverlayCommand {
    fn name() -> &'static str {
        "set_overlay"
    }
    fn execute(
        _terminal: &mut Terminal<NavigatorFile, NavigatorDirectory>,
        args: &[&str],
    ) -> CommandResult {
        let args_owned = args.iter().map(|s| s.to_string()).collect();
        CommandResult::Unhandled(Self::name().to_string(), args_owned)
    }
}

impl NavigatorCommand for SetOverlayCommand {
    fn execute_navigator(navigator: &mut Navigator, args: &[String]) {
        let history: &mut Vec<String> = &mut navigator.terminal.base.history;
        if let Some(overlay) = args.first() {
            if let Ok(layout) = Layout::try_from_name(overlay.as_str()) {
                history.push(format!("Switched to {} overlay", layout.name()));
                navigator.experiment_overlay = Some(layout);
            } else {
                history.push(format!("set_overlay: unknown overlay '{}'", overlay));
            }
        } else {
            navigator.experiment_overlay = None;
            let overlay_names: Vec<String> =
                Layout::ALL.iter().map(|l| l.name().to_string()).collect();
            history.push(format!(
                "Switched to empty overlay, use: set_overlay {:?} for content overlays",
                overlay_names
            ));
        }
    }
}
