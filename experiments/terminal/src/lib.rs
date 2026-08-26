use egui::{Color32, TextStyle};
use std::collections::HashMap;

use crate::style_sheet::{
    BACKGROUND_COLOR, BACKGROUND_CORNER_RADIUS, HOST, ICON, NEOFETCH_SPECS, PROMPT_TEXT_COLOR, SELECTION_COLOR, TEXT_COLOR,
    TEXT_STYLE, USER
};
pub mod style_sheet;

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
pub struct TerminalStyle {
    background_color: Color32,
    background_corner_radius: f32,
    prompt_text_color: Color32,
    selection_color: Color32,
    text_color: Color32,
    text_style: TextStyle,
    user: &'static str,
    host: &'static str,
}

impl Default for TerminalStyle {
    fn default() -> Self {
        Self {
            background_color: BACKGROUND_COLOR,
            background_corner_radius: BACKGROUND_CORNER_RADIUS,
            prompt_text_color: PROMPT_TEXT_COLOR,
            selection_color: SELECTION_COLOR,
            text_color: TEXT_COLOR,
            text_style: TEXT_STYLE,
            user: USER,
            host: HOST,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Terminal {
    pub history: Vec<String>,
    pub input: String,
    pub current_directory: Vec<String>,
    pub file_system: FileSystemNode,
    pub style: TerminalStyle,
}

impl Default for Terminal {
    fn default() -> Self {
        let mut nest_children: HashMap<String, FileSystemNode> = HashMap::new();
        nest_children.insert(
            "readme.txt".to_string(),
            FileSystemNode::File {
                content: "README lol!, bird larping frfr".to_string(),
            },
        );

        let mut nest_directory: HashMap<String, FileSystemNode> = HashMap::new();
        nest_directory.insert(
            "bird".to_string(),
            FileSystemNode::Directory {
                children: nest_children,
            },
        );

        let mut root_children: HashMap<String, FileSystemNode> = HashMap::new();
        root_children.insert(
            "nest".to_string(),
            FileSystemNode::Directory {
                children: nest_directory,
            },
        );

        let mut terminal: Self = Self {
            history: Vec::new(),
            input: String::new(),
            current_directory: vec!["nest".to_string(), "bird".to_string()],
            file_system: FileSystemNode::Directory {
                children: root_children,
            },
            style: TerminalStyle::default(),
        };

        terminal.execute_command("neofetch");

        terminal
    }
}

impl Terminal {
    fn get_node<'a>(&'a self, path_parts_slice: &[String]) -> Option<&'a FileSystemNode> {
        let mut current_node: &FileSystemNode = &self.file_system;

        for path_part in path_parts_slice {
            if path_part.is_empty() {
                continue;
            }
            match current_node {
                FileSystemNode::Directory { children } => {
                    if let Some(child_node) = children.get(path_part) {
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
            self.style.user,
            self.style.host,
            self.current_directory.join("/"),
            raw_command
        );
        self.history.push(prompt);

        let parts: Vec<&str> =
            raw_command.split_whitespace().collect::<Vec<&str>>();

        if parts.is_empty() {
            return CommandResult::Handled;
        }

        let command: &str = parts[0];
        let args: &[&str] = &parts[1..];

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
                    let mut directory_files: Vec<String> =
                        children.keys().cloned().collect::<Vec<String>>();
                    directory_files.sort();
                    self.history.push(directory_files.join("  "));
                }
            }
            "cd" => {
                let default_target_directory: &str = "/";
                let target_directory: &str = args
                    .first()
                    .copied()
                    .unwrap_or(default_target_directory);

                let mut new_path: Vec<String> = if target_directory.starts_with('/') {
                    Vec::new()
                } else {
                    self.current_directory.clone()
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

                if let Some(FileSystemNode::Directory { .. }) = self.get_node(&new_path) {
                    self.current_directory = new_path;
                } else {
                    self.history.push(format!(
                        "cd: {}: No such file or directory",
                        target_directory
                    ));
                }
            }
            "cat" => {
                if let Some(target_file) = args.first() {
                    let mut file_path: Vec<String> = self.current_directory.clone();
                    file_path.push(target_file.to_string());

                    if let Some(FileSystemNode::File { content }) = self.get_node(&file_path) {
                        self.history.push(content.clone());
                    } else {
                        self.history
                            .push(format!("cat: {}: No such file", target_file));
                    }
                } else {
                    self.history.push("cat: missing file operand".to_string());
                }
            }
            "neofetch" => {
                let mut os_info: Vec<String> = vec![
                    format!("\x1b[1;36m{}@{}\x1b[0m", self.style.user, self.style.host),
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

                let start_offset_index: usize = (icon_lines
                    .len()
                    .saturating_sub(os_info.len()))
                    / 2;

                let mut info: std::vec::IntoIter<String> =
                    os_info.into_iter();

                for (line_index, line) in icon_lines.into_iter().enumerate() {
                    let padded_icon: String =
                        format!("{:<width$}", line, width = maximum_width + 4);

                    if line_index >= start_offset_index && let Some(info_text) = info.next() {
                        self.history
                            .push(format!("{}{}", padded_icon, info_text));
                        continue;
                    }
                    self.history.push(padded_icon);
                }
            }
            "help" => {
                self.history.push(
                    "Available built-in commands: clear, pwd, ls, cd, cat, neofetch, help"
                        .to_string(),
                );
            }
            _ => {
                let unhandled_arguments: Vec<String> = args
                    .iter()
                    .map(|string_reference: &&str| -> String { string_reference.to_string() })
                    .collect::<Vec<String>>();

                return CommandResult::Unhandled(
                    command.to_string(),
                    unhandled_arguments,
                );
            }
        }

        CommandResult::Handled
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<CommandResult> {
        let mut final_command_result: Option<CommandResult> = None;

        let mut style: egui::Style = (**ui.style()).clone();
        style.visuals.override_text_color = Some(self.style.text_color);
        style.visuals.selection.bg_fill = self.style.selection_color;
        ui.set_style(style);

        ui.painter().rect_filled(
            ui.available_rect_before_wrap(),
            self.style.background_corner_radius,
            self.style.background_color,
        );

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .stick_to_bottom(true)
            .show(ui, |ui: &mut egui::Ui| -> () {
                for line in &self.history {
                    if line.contains("\x1b[") {
                        let mut layout_job: egui::text::LayoutJob =
                            egui::text::LayoutJob::default();

                        let mut current_text_format: egui::text::TextFormat =
                            egui::text::TextFormat {
                                font_id: TEXT_STYLE.resolve(ui.style()),
                                color: self.style.text_color,
                                background: egui::Color32::TRANSPARENT,
                                italics: false,
                                underline: egui::Stroke::NONE,
                                strikethrough: egui::Stroke::NONE,
                                ..Default::default()
                            };
                        let mut remaining: &str = line.as_str();

                        while let Some(escape_sequence_start_index) = remaining.find("\x1b[")
                        {
                            if escape_sequence_start_index > 0 {
                                layout_job.append(
                                    &remaining[..escape_sequence_start_index],
                                    0.0,
                                    current_text_format.clone(),
                                );
                            }

                            let string_after_escape: &str =
                                &remaining[escape_sequence_start_index + 2..];
                            if let Some(letter_m_index) = string_after_escape.find('m') {
                                let codes: &str = &string_after_escape[..letter_m_index];

                                let code_parts: std::str::Split<char> =
                                    codes.split(';');

                                for code_part in code_parts {
                                    crate::style_sheet::apply_ansi_code(
                                        code_part,
                                        &mut current_text_format,
                                        self.style.text_color,
                                        egui::Color32::TRANSPARENT,
                                    );
                                }

                                remaining = &string_after_escape[letter_m_index + 1..];
                            } else {
                                break;
                            }
                        }

                        if !remaining.is_empty() {
                            layout_job.append(remaining, 0.0, current_text_format.clone());
                        }

                        ui.add(egui::Label::new(layout_job));
                    } else {
                        ui.add(egui::Label::new(
                            egui::RichText::new(line).text_style(TEXT_STYLE),
                        ));
                    }
                }

                ui.horizontal(|horizontal_ui: &mut egui::Ui| -> () {
                    let prompt: String = format!(
                        "[{}@{} /{}]$",
                        self.style.user,
                        self.style.host,
                        self.current_directory.join("/")
                    );

                    horizontal_ui.add(egui::Label::new(
                        egui::RichText::new(&prompt)
                            .text_style(TEXT_STYLE)
                            .color(self.style.prompt_text_color),
                    ));

                    let response: egui::Response = horizontal_ui.add(
                        egui::TextEdit::singleline(&mut self.input)
                            .font(TEXT_STYLE)
                            .frame(egui::Frame::NONE)
                            .desired_width(f32::INFINITY),
                    );

                    response.request_focus();

                    let enter_key_pressed: bool =
                        horizontal_ui.input(|input_state: &egui::InputState| -> bool {
                            input_state.key_pressed(egui::Key::Enter)
                        });

                    if response.has_focus() && enter_key_pressed {
                        let command: String = self.input.clone();
                        self.input.clear();

                        if !command.trim().is_empty() {
                            let exec_result: CommandResult =
                                self.execute_command(&command);

                            if let CommandResult::Unhandled(..) = exec_result {
                                final_command_result = Some(exec_result);
                            }
                        }

                        response.request_focus();
                    }
                });
            });

        final_command_result
    }
}

impl eframe::App for Terminal {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.ui(ui);
    }
}
