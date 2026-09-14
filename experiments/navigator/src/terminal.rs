use crate::{
    graph::{GraphInteractNodeIndex, GraphMode},
    layouts::{ExperimentIndex, Layout},
    navigator::{Navigator, NavigatorUpdate},
    settings::style_sheet::TERMINAL_STYLE,
};
use std::collections::HashMap;
use terminal::{
    app::{AppCommand, AppTerminal},
    command::{ClearCommand, HelpCommand},
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

pub type NavigatorTerminal = AppTerminal<NavigatorFile, NavigatorDirectory, Navigator>;

pub fn create_navigator_terminal() -> NavigatorTerminal {
    let root = FileSystemNode::Directory(NavigatorDirectory::default());
    let mut base = Terminal::new_empty(root);
    base.register_command::<HelpCommand>();
    base.register_command::<ClearCommand>();
    base.style = TERMINAL_STYLE;

    let mut terminal = NavigatorTerminal::new(base);

    terminal.register_app_command::<SetModeCommand>();
    terminal.register_app_command::<SetOverlayCommand>();
    terminal.register_app_command::<OpenExperimentCommand>();

    terminal.base.execute_command("help");
    terminal
}

pub struct SetModeCommand;

impl AppCommand<Navigator> for SetModeCommand {
    fn name() -> &'static str {
        "set_mode"
    }

    fn execute_app(navigator: &mut Navigator, args: &[String]) {
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

        SetOverlayCommand::execute_app(navigator, &Vec::new());
    }
}

pub struct SetOverlayCommand;

impl AppCommand<Navigator> for SetOverlayCommand {
    fn name() -> &'static str {
        "set_overlay"
    }
    fn execute_app(navigator: &mut Navigator, args: &[String]) {
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

impl AppCommand<Navigator> for OpenExperimentCommand {
    fn name() -> &'static str {
        "open_ex"
    }
    fn execute_app(navigator: &mut Navigator, args: &[String]) {
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
