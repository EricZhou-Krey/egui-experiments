use eframe::egui;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum FileSystemNode {
    File {
        content: String,
    },
    Directory {
        children: HashMap<String, FileSystemNode>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandResult {
    Handled,
    Unhandled(String, Vec<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Terminal {
    pub history: Vec<String>,
    pub input: String,
    pub current_directory: Vec<String>,
    pub file_system: FileSystemNode,
    pub user: String,
    pub host: String,
}

impl Default for Terminal {
    fn default() -> Self {
        let mut home_children: HashMap<String, FileSystemNode> = HashMap::new();
        home_children.insert(
            "readme.txt".to_string(),
            FileSystemNode::File {
                content: "Welcome to the emulator!".to_string(),
            },
        );

        let mut home_directory: HashMap<String, FileSystemNode> = HashMap::new();
        home_directory.insert(
            "user".to_string(),
            FileSystemNode::Directory {
                children: home_children,
            },
        );

        let mut root_children: HashMap<String, FileSystemNode> = HashMap::new();
        root_children.insert(
            "home".to_string(),
            FileSystemNode::Directory {
                children: home_directory,
            },
        );
        root_children.insert(
            "etc".to_string(),
            FileSystemNode::Directory {
                children: HashMap::new(),
            },
        );

        let mut terminal: Self = Self {
            history: Vec::new(),
            input: String::new(),
            current_directory: vec!["home".to_string(), "user".to_string()],
            file_system: FileSystemNode::Directory {
                children: root_children,
            },
            user: "user".to_string(),
            host: "archlinux".to_string(),
        };

        terminal.execute_command("neofetch");

        terminal
    }
}

impl Terminal {
    fn get_node<'a>(&'a self, path_parts: &[String]) -> Option<&'a FileSystemNode> {
        let mut current_node: &FileSystemNode = &self.file_system;

        for part in path_parts {
            if part.is_empty() {
                continue;
            }
            match current_node {
                FileSystemNode::Directory { children } => {
                    if let Some(child_node) = children.get(part) {
                        current_node = child_node;
                    } else {
                        return None;
                    }
                }
                FileSystemNode::File { .. } => return None,
            }
        }

        Some(current_node)
    }

    pub fn execute_command(&mut self, raw_command: &str) -> CommandResult {
        let prompt: String = format!(
            "[{}@{} /{}]$ {}",
            self.user,
            self.host,
            self.current_directory.join("/"),
            raw_command
        );
        self.history.push(prompt);

        let parts: Vec<&str> = raw_command.split_whitespace().collect::<Vec<&str>>();

        if parts.is_empty() {
            return CommandResult::Handled;
        }

        let command: &str = parts[0];
        let arguments: &[&str] = &parts[1..];

        match command {
            "clear" => {
                self.history.clear();
            }
            "pwd" => {
                let path: String = format!("/{}", self.current_directory.join("/"));
                self.history.push(path);
            }
            "ls" => {
                if let Some(FileSystemNode::Directory { children }) =
                    self.get_node(&self.current_directory)
                {
                    let mut entries: Vec<String> =
                        children.keys().cloned().collect::<Vec<String>>();
                    entries.sort();
                    self.history.push(entries.join("  "));
                }
            }
            "cd" => {
                let default_target: &str = "/";
                let target: &str = arguments.first().copied().unwrap_or(default_target);

                let mut new_path: Vec<String> = if target.starts_with('/') {
                    Vec::new()
                } else {
                    self.current_directory.clone()
                };

                for part in target.split('/') {
                    match part {
                        "" | "." => {}
                        ".." => {
                            new_path.pop();
                        }
                        _ => new_path.push(part.to_string()),
                    }
                }

                if let Some(FileSystemNode::Directory { .. }) = self.get_node(&new_path) {
                    self.current_directory = new_path;
                } else {
                    self.history
                        .push(format!("cd: {}: No such file or directory", target));
                }
            }
            "cat" => {
                if let Some(target) = arguments.first() {
                    let mut file_path: Vec<String> = self.current_directory.clone();
                    file_path.push(target.to_string());

                    if let Some(FileSystemNode::File { content }) = self.get_node(&file_path) {
                        self.history.push(content.clone());
                    } else {
                        self.history.push(format!("cat: {}: No such file", target));
                    }
                } else {
                    self.history.push("cat: missing file operand".to_string());
                }
            }
            "neofetch" => {
                let ascii_art: &str = r#"
       /\         OS: Linux x86_64
      /  \        Host: eframe-terminal-emulator
     /____\       Kernel: 6.x.x-emulated
    /      \      Uptime: 0 mins
   /        \     Shell: custom-rust-sh
  /          \    DE: egui
                "#;
                for line in ascii_art.trim_matches('\n').lines() {
                    self.history.push(line.to_string());
                }
            }
            "help" => {
                self.history.push(
                    "Available built-in commands: clear, pwd, ls, cd, cat, neofetch, help"
                        .to_string(),
                );
            }
            _ => {
                let unhandled_arguments: Vec<String> = arguments
                    .iter()
                    .map(|string_reference: &&str| -> String { string_reference.to_string() })
                    .collect::<Vec<String>>();

                return CommandResult::Unhandled(command.to_string(), unhandled_arguments);
            }
        }

        CommandResult::Handled
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<CommandResult> {
        let mut result: Option<CommandResult> = None;

        let mut style: egui::Style = (**ui.style()).clone();
        style.visuals.override_text_color = Some(egui::Color32::from_rgb(200, 200, 200));
        style.visuals.selection.bg_fill = egui::Color32::from_rgba_unmultiplied(100, 100, 100, 100);
        ui.set_style(style);

        ui.painter().rect_filled(
            ui.available_rect_before_wrap(),
            0.0,
            egui::Color32::from_rgb(15, 15, 15),
        );

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .stick_to_bottom(true)
            .show(ui, |scroll_ui: &mut egui::Ui| -> () {
                scroll_ui.add_space(4.0);

                for line in &self.history {
                    scroll_ui.add(egui::Label::new(
                        egui::RichText::new(line).family(egui::FontFamily::Monospace),
                    ));
                }

                scroll_ui.horizontal(|horizontal_ui: &mut egui::Ui| -> () {
                    let prompt: String = format!(
                        "[{}@{} /{}]$",
                        self.user,
                        self.host,
                        self.current_directory.join("/")
                    );

                    horizontal_ui.add(egui::Label::new(
                        egui::RichText::new(&prompt)
                            .family(egui::FontFamily::Monospace)
                            .color(egui::Color32::from_rgb(80, 250, 120)),
                    ));

                    let response: egui::Response = horizontal_ui.add(
                        egui::TextEdit::singleline(&mut self.input)
                            .font(egui::TextStyle::Monospace)
                            .frame(egui::Frame::NONE)
                            .desired_width(f32::INFINITY),
                    );

                    response.request_focus();

                    let enter_pressed: bool =
                        horizontal_ui.input(|input_state: &egui::InputState| -> bool {
                            input_state.key_pressed(egui::Key::Enter)
                        });

                    if response.has_focus() && enter_pressed {
                        let command_string: String = self.input.clone();
                        self.input.clear();

                        if !command_string.trim().is_empty() {
                            let execution_result: CommandResult =
                                self.execute_command(&command_string);

                            if let CommandResult::Unhandled(..) = execution_result {
                                result = Some(execution_result);
                            }
                        }

                        response.request_focus();
                    }
                });
            });

        result
    }
}

impl eframe::App for Terminal {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.ui(ui);
    }
}
