use terminal::{
    command::{Command, CommandResult},
    file_system::{FileSystemNode, TerminalFile},
    Terminal,
};

use crate::{
    state::TTSState,
    terminal::file_system::{TTSDirectory, TTSFile},
};

pub type TTSFn = fn(&mut TTSState, &[String]);

pub trait TTSCommand: Command<TTSFile, TTSDirectory> {
    fn execute_tts(tts: &mut TTSState, args: &[String]);
}

pub struct TTSCatCommand;
impl Command<TTSFile, TTSDirectory> for TTSCatCommand {
    fn name() -> &'static str {
        "cat"
    }
    fn execute(terminal: &mut Terminal<TTSFile, TTSDirectory>, args: &[&str]) -> CommandResult {
        if let Some(target_file) = args.first() {
            let mut file_path: Vec<String> = terminal.current_directory.clone();
            file_path.push(target_file.to_string());

            if let Some(FileSystemNode::File(file)) = terminal.get_node(&file_path) {
                match file {
                    TTSFile::Terminal(terminal_file) => match terminal_file {
                        TerminalFile::Text(text_file) => {
                            terminal.history.push(text_file.content.clone());
                        }
                        TerminalFile::Binary(..) => {
                            terminal
                                .history
                                .push(format!("cat: {}: cannot display binary file", target_file));
                        }
                    },
                    TTSFile::SceneObject(..) => {
                        let args_owned = args.iter().map(|s| s.to_string()).collect();
                        return CommandResult::Unhandled(Self::name().to_string(), args_owned);
                    }
                }
            } else {
                terminal
                    .history
                    .push(format!("cat: {}: No such file", target_file));
            }
        } else {
            terminal
                .history
                .push("cat: missing file operand".to_string());
        }
        CommandResult::Handled
    }
}

impl TTSCommand for TTSCatCommand {
    fn execute_tts(tts: &mut TTSState, args: &[String]) {
        todo!()
    }
}
