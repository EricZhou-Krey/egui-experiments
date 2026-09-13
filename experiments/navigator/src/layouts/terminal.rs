use crate::{
    layouts::{to_rect, UiFn},
    settings::style_sheet::LAYOUT_BLOCK_FRAME,
};
use egui::Rect;

const TITLE_RECT: (f32, f32, f32, f32) = (0.0, 0.0, 0.4, 0.2);
const DESC_RECT: (f32, f32, f32, f32) = (0.0, 0.2, 0.4, 1.0);
const PREVIEW_RECT: (f32, f32, f32, f32) = (0.4, 0.0, 1.0, 0.5);

pub fn panels(max_rect: Rect) -> Vec<(Rect, UiFn)> {
    vec![
        (to_rect(TITLE_RECT, max_rect), title_content),
        (to_rect(DESC_RECT, max_rect), description_content),
        (to_rect(PREVIEW_RECT, max_rect), preview_content),
    ]
}

fn title_content(ui: &mut egui::Ui) {
    egui::CentralPanel::default()
        .frame(LAYOUT_BLOCK_FRAME)
        .show(ui, |ui| {
            ui.centered_and_justified(|ui| ui.heading("TERMINAL EMULATOR"));
        });
}

fn description_content(ui: &mut egui::Ui) {
    egui::CentralPanel::default()
        .frame(LAYOUT_BLOCK_FRAME)
        .show(ui, |ui| {
            egui::ScrollArea::vertical().id_salt("terminal_desc_scroll").show(ui, |ui| {
                ui.heading("PROJECT PURPOSE");
                ui.separator();
                
                ui.label("This experiment features a fully integrated, modular terminal emulator built entirely within egui. It serves as an interactive command-line interface tailored for the application context.");
                ui.add_space(10.0);
                
                ui.label("Rather than simply forwarding commands to the host OS, it operates on its own self-contained memory-backed Virtual File System (VFS). The architecture heavily utilizes Rust's trait system and generics, allowing seamless expansion of both the file system node types and the executable command registry.");
                ui.add_space(10.0);
                
                ui.label("Unhandled commands gracefully pass execution requests back to the parent layout, making this highly effective as a master control console for the broader UI interface.");
            });
        });
}

fn preview_content(ui: &mut egui::Ui) {
    egui::CentralPanel::default()
        .frame(LAYOUT_BLOCK_FRAME)
        .show(ui, |ui| {
            egui::ScrollArea::vertical().id_salt("terminal_preview_scroll").show(ui, |ui| {
                ui.heading("TECHNICAL SPECIFICATIONS");
                ui.separator();
                
                ui.label(egui::RichText::new("Virtual File System (VFS)").strong());
                ui.label("• Generic Node enums (Directory / File)\n• Native support for Plain Text & Binary data storage\n• Trait-based hierarchy navigable via absolute and relative paths");
                
                ui.add_space(15.0);
                
                ui.label(egui::RichText::new("Built-In Commands").strong());
                ui.label("• File Navigation: cd, ls (with -a flag), pwd\n• Execution & IO: cat, clear, help\n• System Info: neofetch (with embedded ASCII art)");
                
                ui.add_space(15.0);
                
                ui.label(egui::RichText::new("UI & Parser Features").strong());
                ui.label("• Real-time ANSI escape sequence rendering for inline text coloring\n• Persistent command history navigated via Up/Down arrow keys\n• Auto-scrolling and intelligent focus locking mechanism");
            });
        });
}
