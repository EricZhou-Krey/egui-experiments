use crate::Terminal;
use crate::style_sheet::{ICON, ICON_PADDING, NEOFETCH_SPECS};
use crate::file_system::{Directory, FileSystemNode, TerminalFile}; 

#[derive(Debug, Clone, PartialEq)]
pub enum CommandResult {
    Handled,
    Unhandled(String, Vec<String>),
}

pub trait Command<F, D> {
    fn name() -> &'static str;
    fn execute(terminal: &mut Terminal<F, D>, args: &[&str]) -> CommandResult;
}

pub type CommandFn<F, D> = fn(&mut Terminal<F, D>, &[&str]) -> CommandResult;

pub struct ClearCommand;
impl<F, D> Command<F, D> for ClearCommand {
    fn name() -> &'static str { "clear" }
    fn execute(terminal: &mut Terminal<F, D>, _args: &[&str]) -> CommandResult {
        terminal.history.clear();
        CommandResult::Handled
    }
}

pub struct PwdCommand;
impl<F, D> Command<F, D> for PwdCommand {
    fn name() -> &'static str { "pwd" }
    fn execute(terminal: &mut Terminal<F, D>, _args: &[&str]) -> CommandResult {
        let path: String = format!("/{}", terminal.current_directory.join("/"));
        terminal.history.push(path);
        CommandResult::Handled
    }
}

pub struct LsCommand;
impl<F, D> Command<F, D> for LsCommand
where
    D: Directory<Node = FileSystemNode<F, D>>,
{
    fn name() -> &'static str { "ls" }
    fn execute(terminal: &mut Terminal<F, D>, args: &[&str]) -> CommandResult {
        let show_all = args.contains(&"-a");

        if let Some(FileSystemNode::Directory(directory)) = terminal.get_node(&terminal.current_directory) {
            let mut directory_files: Vec<String> = directory
                .children()
                .into_iter()
                .filter(|file_name| show_all || !file_name.starts_with('.'))
                .collect();

            if show_all {
                directory_files.push(".".to_string());
                directory_files.push("..".to_string());
            }

            directory_files.sort();
            terminal.history.push(directory_files.join("  "));
        }
        CommandResult::Handled
    }
}

pub struct CdCommand;
impl<F, D> Command<F, D> for CdCommand
where
    D: Directory<Node = FileSystemNode<F, D>>,
{
    fn name() -> &'static str { "cd" }
    fn execute(terminal: &mut Terminal<F, D>, args: &[&str]) -> CommandResult {
        let default_target_directory: &str = "/";
        let target_directory: &str = args
            .first()
            .copied()
            .unwrap_or(default_target_directory);

        let mut new_path: Vec<String> = if target_directory.starts_with('/') {
            Vec::new()
        } else {
            terminal.current_directory.clone()
        };

        for path_part in target_directory.split('/') {
            match path_part {
                "" | "." => {}
                ".." => {
                    new_path.pop();
                }
                _ => new_path.push(path_part.to_string()),
            }
        }

        if let Some(FileSystemNode::Directory(_)) = terminal.get_node(&new_path) {
            terminal.current_directory = new_path;
        } else {
            terminal.history.push(format!(
                "cd: {}: No such file or directory",
                target_directory
            ));
        }
        CommandResult::Handled
    }
}

pub struct CatCommand;
impl<D> Command<TerminalFile, D> for CatCommand
where
    D: Directory<Node = FileSystemNode<TerminalFile, D>>,
{
    fn name() -> &'static str { "cat" }
    fn execute(terminal: &mut Terminal<TerminalFile, D>, args: &[&str]) -> CommandResult {
        if let Some(target_file) = args.first() {
            let mut file_path: Vec<String> = terminal.current_directory.clone();
            file_path.push(target_file.to_string());

            if let Some(FileSystemNode::File(file)) = terminal.get_node(&file_path) {
                match file {
                    TerminalFile::Text(text_file) => {
                        terminal.history.push(text_file.content.clone());
                    }
                    TerminalFile::Binary(_) => {
                        terminal.history.push(format!("cat: {}: cannot display binary file", target_file));
                    }
                }
            } else {
                terminal.history.push(format!("cat: {}: No such file", target_file));
            }
        } else {
            terminal.history.push("cat: missing file operand".to_string());
        }
        CommandResult::Handled
    }
}

pub struct NeofetchCommand;
impl<F, D> Command<F, D> for NeofetchCommand {
    fn name() -> &'static str { "neofetch" }
    fn execute(terminal: &mut Terminal<F, D>, _args: &[&str]) -> CommandResult {
        let mut os_info: Vec<String> = vec![
            format!("\x1b[1;36m{}@{}\x1b[0m", terminal.style.user, terminal.style.host),
        ];

        for spec in NEOFETCH_SPECS {
            os_info.push(spec.to_string());
        }

        let icon_lines: Vec<&str> = ICON
            .lines()
            .skip_while(|line: &&str| -> bool { line.is_empty() })
            .collect();

        let maximum_width: usize = icon_lines
            .iter()
            .map(|line: &&str| -> usize { line.chars().count() })
            .max()
            .unwrap_or(0);

        let start_offset_index: usize = (icon_lines.len().saturating_sub(os_info.len())) / 2;

        let mut info: std::vec::IntoIter<String> = os_info.into_iter();

        for (line_index, line) in icon_lines.into_iter().enumerate() {
            let padded_icon: String = format!("{:<width$}", line, width = maximum_width + ICON_PADDING);

            if line_index >= start_offset_index && let Some(info_text) = info.next() {
                terminal.history.push(format!("{}{}", padded_icon, info_text));
                continue;
            }
            terminal.history.push(padded_icon);
        }
        CommandResult::Handled
    }
}

pub struct HelpCommand;
impl<F, D> Command<F, D> for HelpCommand {
    fn name() -> &'static str { "help" }
    fn execute(terminal: &mut Terminal<F, D>, _args: &[&str]) -> CommandResult {
        let command_strings: Vec<String> = terminal.commands.keys().cloned().collect();
        terminal.history.push(
            format!("Available built-in commands: {}", command_strings.join(", "))
        );
        CommandResult::Handled
    }
}
