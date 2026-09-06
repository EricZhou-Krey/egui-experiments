use terminal::{
    command::{Command, CommandResult},
    file_system::{File, TerminalDirectory, TerminalFile},
    Terminal,
};

pub enum NavigatorFile {
    Terminal(TerminalFile),
}

impl File for NavigatorFile {}

pub fn new_terminal() -> Terminal<NavigatorFile, TerminalDirectory> {
    let terminal = Terminal::default();

    terminal
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
