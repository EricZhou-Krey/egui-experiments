use crate::{
    graph::{GraphInteractNodeIndex, GraphMode},
    layouts::{ExperimentIndex, Layout},
    navigator::{Navigator, NavigatorUpdate},
    settings::style_sheet::TERMINAL_STYLE,
};
use std::collections::HashMap;
use terminal::{
    command::{ClearCommand, Command, CommandResult, HelpCommand},
    file_system::{Directory, File, FileSystemNode, TerminalFile},
    Terminal,
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
        terminal.register_command::<OpenExperimentCommand>();

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
        if let Some(mode_name) = args.first() {
            if let Ok(mode) = GraphMode::try_from_name(mode_name.as_str()) {
                history.push(format!("Switched to {} mode.", mode.name()));
                navigator.graph = Navigator::create_graph(&mode);
                navigator.graph_mode = mode;
            } else {
                history.push(format!("set_mode: unknown mode '{}'", mode_name));
            }
        } else {
            let mode_names: Vec<String> = GraphMode::ALL
                .iter()
                .map(|m| m.name().to_string())
                .collect();
            history.push(format!("Usage: set_mode {:?}", mode_names));
        }

        SetOverlayCommand::execute_navigator(navigator, &Vec::new());
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
        if let Some(overlay_name) = args.first() {
            if let Ok(layout) = Layout::try_from_name(overlay_name.as_str()) {
                history.push(format!("Switched to {} overlay", layout.name()));
                navigator
                    .graph
                    .set_interact_index(Some(GraphInteractNodeIndex(layout.clone() as usize)));
                navigator.experiment_overlay = Some(layout);
            } else {
                history.push(format!("set_overlay: unknown overlay '{}'", overlay_name));
            }
        } else {
            navigator.graph.set_interact_index(None);
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

pub struct OpenExperimentCommand;

impl Command<NavigatorFile, NavigatorDirectory> for OpenExperimentCommand {
    fn name() -> &'static str {
        "open_ex"
    }
    fn execute(
        _terminal: &mut Terminal<NavigatorFile, NavigatorDirectory>,
        args: &[&str],
    ) -> CommandResult {
        let args_owned = args.iter().map(|s| s.to_string()).collect();
        CommandResult::Unhandled(Self::name().to_string(), args_owned)
    }
}

impl NavigatorCommand for OpenExperimentCommand {
    fn execute_navigator(navigator: &mut Navigator, args: &[String]) {
        let history: &mut Vec<String> = &mut navigator.terminal.base.history;
        if let Some(experiment_name) = args.first() {
            if let Ok(experiment_index) = ExperimentIndex::try_from_name(experiment_name.as_str()) {
                navigator.update_status = Some(NavigatorUpdate::Open(experiment_index));
                history.push(format!("Opened {}", experiment_name));
            } else {
                history.push(format!(
                    "open_ex: unknown experiment '{}', use overlay names",
                    experiment_name
                ));
            }
        } else {
            let experiment_names: Vec<String> = ExperimentIndex::ALL
                .iter()
                .map(|l| l.name().to_string())
                .collect();
            history.push(format!("Usage: open_ex {:?}", experiment_names));
        }
    }
}
