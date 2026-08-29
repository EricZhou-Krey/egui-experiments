use egui::{Color32, TextStyle};
use std::collections::HashMap;

use crate::{
    command::{
        CatCommand, CdCommand, ClearCommand, Command, CommandFn, CommandResult, HelpCommand,
        LsCommand, NeofetchCommand, PwdCommand,
    },
    file_system::{Directory, FileSystemNode, TerminalDirectory, TerminalFile, TextFile},
    style_sheet::{
        BACKGROUND_COLOR, BACKGROUND_CORNER_RADIUS, HOST, PROMPT_TEXT_COLOR, SELECTION_COLOR,
        TEXT_COLOR, TEXT_STYLE, USER,
    },
};

pub mod command;
pub mod file_system;
pub mod style_sheet;

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
pub struct Terminal<F, D> {
    pub history: Vec<String>,
    pub input: String,
    pub current_directory: Vec<String>,
    pub file_system: FileSystemNode<F, D>,
    pub style: TerminalStyle,
    pub commands: HashMap<String, CommandFn<F, D>>,
}

impl Default for Terminal<TerminalFile, TerminalDirectory> {
    fn default() -> Self {
        let mut nest_children: HashMap<String, FileSystemNode<TerminalFile, TerminalDirectory>> =
            HashMap::new();
        nest_children.insert(
            "readme.txt".to_string(),
            FileSystemNode::File(TerminalFile::Text(TextFile {
                content: "README lol!, bird larping frfr".to_string(),
            })),
        );

        let mut nest_directory: HashMap<String, FileSystemNode<TerminalFile, TerminalDirectory>> =
            HashMap::new();
        nest_directory.insert(
            "bird".to_string(),
            FileSystemNode::Directory(TerminalDirectory {
                children: nest_children,
            }),
        );

        let mut root_children: HashMap<String, FileSystemNode<TerminalFile, TerminalDirectory>> =
            HashMap::new();
        root_children.insert(
            "nest".to_string(),
            FileSystemNode::Directory(TerminalDirectory {
                children: nest_directory,
            }),
        );

        let mut terminal: Self = Self {
            history: Vec::new(),
            input: String::new(),
            current_directory: Vec::new(),
            file_system: FileSystemNode::Directory(TerminalDirectory {
                children: root_children,
            }),
            style: TerminalStyle::default(),
            commands: HashMap::new(),
        };

        terminal.register_command::<ClearCommand>();
        terminal.register_command::<PwdCommand>();
        terminal.register_command::<LsCommand>();
        terminal.register_command::<CdCommand>();
        terminal.register_command::<CatCommand>();
        terminal.register_command::<NeofetchCommand>();
        terminal.register_command::<HelpCommand>();

        terminal.execute_command("neofetch");

        terminal
    }
}

impl<F, D> Terminal<F, D>
where
    D: Directory<Node = FileSystemNode<F, D>>,
{
    pub fn get_node<'a>(&'a self, path_parts_slice: &[String]) -> Option<&'a FileSystemNode<F, D>> {
        let mut current_node: &FileSystemNode<F, D> = &self.file_system;

        for path_part in path_parts_slice {
            if path_part.is_empty() {
                continue;
            }
            match current_node {
                FileSystemNode::Directory(dir) => {
                    if let Some(child_node) = dir.child(path_part) {
                        current_node = child_node;
                    } else {
                        return None;
                    }
                }
                FileSystemNode::File(_) => return None,
            }
        }

        Some(current_node)
    }

    pub fn register_command<C: Command<F, D>>(&mut self) {
        self.commands.insert(C::name().to_string(), C::execute);
    }

    pub fn remove_command(&mut self, name: &str) {
        self.commands.remove(name);
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

        let parts: Vec<&str> = raw_command.split_whitespace().collect::<Vec<&str>>();

        if parts.is_empty() {
            return CommandResult::Handled;
        }

        let command: &str = parts[0];
        let args: &[&str] = &parts[1..];

        let command_function_option: Option<CommandFn<F, D>> = self.commands.get(command).copied();

        if let Some(command_function) = command_function_option {
            command_function(self, args)
        } else {
            let unhandled_arguments: Vec<String> = args
                .iter()
                .map(|string_reference: &&str| -> String { string_reference.to_string() })
                .collect::<Vec<String>>();

            CommandResult::Unhandled(command.to_string(), unhandled_arguments)
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<CommandResult> {
        let mut final_command_result: Option<CommandResult> = None;

        let mut style: egui::Style = (**ui.style()).clone();
        style.visuals.override_text_color = Some(self.style.text_color);
        style.visuals.selection.bg_fill = self.style.selection_color;
        ui.set_style(style);

        let terminal_rect: egui::Rect = ui.available_rect_before_wrap();

        ui.painter().rect_filled(
            terminal_rect,
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

                        while let Some(escape_sequence_start_index) = remaining.find("\x1b[") {
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

                                let code_parts: std::str::Split<char> = codes.split(';');

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

                let mut terminal_clicked: bool = false;
                ui.input(|input: &egui::InputState| {
                    if input.pointer.primary_clicked() && let Some(pos) = input.pointer.interact_pos() {
                        terminal_clicked = terminal_rect.contains(pos);
                    }
                });

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
                            .id_source("terminal_input_id")
                            .font(TEXT_STYLE)
                            .frame(egui::Frame::NONE)
                            .desired_width(f32::INFINITY)
                            .lock_focus(true),
                    );

                    if terminal_clicked {
                        response.request_focus();

                    }

                    if response.lost_focus() && horizontal_ui.input(|input: &egui::InputState| -> bool {
                            input.key_pressed(egui::Key::Enter)
                        })
                    {
                        let command: String = self.input.clone();
                        self.input.clear();

                        if !command.trim().is_empty() {
                            let exec_result: CommandResult = self.execute_command(&command);

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

impl eframe::App for Terminal<TerminalFile, TerminalDirectory> {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.ui(ui);
    }
}
