use terminal::{
    file_system::{TerminalDirectory, TerminalFile},
    Terminal,
};

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };

    eframe::run_native(
        "egui-experiments",
        native_options,
        Box::new(|_cc| {
            Ok(Box::new(
                Terminal::<TerminalFile, TerminalDirectory>::example(),
            ))
        }),
    )
}
