use terminal::{
    app::AppCommand,
    file_system::{FileSystemNode, TerminalFile},
};

use crate::{state::TTSState, terminal::file_system::TTSFile};

pub struct TTSInfoCommand;

impl AppCommand<TTSState> for TTSInfoCommand {
    fn name() -> &'static str {
        "info"
    }

    fn execute_app(tts: &mut TTSState, args: &[String]) {
        let terminal = &mut tts.terminal.base;

        if let Some(target_file) = args.first() {
            let mut file_path: Vec<String> = terminal.current_directory.clone();
            file_path.push(target_file.to_string());

            let node_option = terminal.get_node(&file_path).cloned();

            if let Some(FileSystemNode::File(file)) = node_option {
                match file {
                    TTSFile::Terminal(terminal_file) => match terminal_file {
                        TerminalFile::Text(text_file) => {
                            terminal.history.push(text_file.content);
                        }
                        TerminalFile::Binary(..) => {
                            terminal
                                .history
                                .push(format!("info: {}: cannot display binary file", target_file));
                        }
                    },
                    TTSFile::SceneObject(object_key) => {
                        terminal.history.push(format!(
                            "Successfully read SceneObject {:?} from state!",
                            object_key
                        ));
                    }
                }
            } else {
                terminal
                    .history
                    .push(format!("info: {}: No such file", target_file));
            }
        } else {
            terminal
                .history
                .push("info: missing file operand".to_string());
        }
    }
}
