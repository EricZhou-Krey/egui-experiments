use crate::command::{Command, CommandResult};
use crate::file_system::{Directory, FileSystemNode};
use crate::Terminal;
use std::collections::HashMap;

pub type AppCommandFn<S> = fn(&mut S, &[String]);
pub type PendingCommand<S> = (AppCommandFn<S>, Vec<String>);

pub trait AppCommand<S> {
    fn name() -> &'static str;
    fn execute_app(state: &mut S, args: &[String]);
}

fn auto_bubbler<C, S, F, D>(_terminal: &mut Terminal<F, D>, args: &[&str]) -> CommandResult
where
    C: AppCommand<S>,
{
    let args_owned = args.iter().map(|s| s.to_string()).collect();
    CommandResult::Unhandled(C::name().to_string(), args_owned)
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppTerminal<F, D, S> {
    pub base: Terminal<F, D>,
    pub commands: HashMap<String, AppCommandFn<S>>,
}

impl<F, D, S> AppTerminal<F, D, S>
where
    D: Directory<Node = FileSystemNode<F, D>>,
{
    pub fn new(base: Terminal<F, D>) -> Self {
        Self {
            base,
            commands: HashMap::new(),
        }
    }

    pub fn register_base_command<C: Command<F, D>>(&mut self) {
        self.base.register_command::<C>();
    }

    pub fn register_app_command<C: AppCommand<S>>(&mut self) {
        let name = C::name().to_string();

        self.base
            .commands
            .insert(name.clone(), auto_bubbler::<C, S, F, D>);

        self.commands.insert(name, C::execute_app);
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<PendingCommand<S>> {
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
