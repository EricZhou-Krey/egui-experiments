use eframe::egui;
use terminal::{
    file_system::{TerminalDirectory, TerminalFile},
    Terminal,
};

use crate::{
    boids::graph::BoidGraph,
    life::graph::LifeGraph,
    settings::{
        style_sheet::{MIN_TERMINAL_SIZE, TERMINAL_STYLE},
        NavigatorSettings,
    },
    triangulation::graph::TriangulationGraph,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Tab {
    Settings,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum GraphMode {
    #[default]
    Triangulation,
    Boids,
    Life,
}

pub enum Graph {
    Triangulation(Box<TriangulationGraph>),
    Boids(Box<BoidGraph>),
    Life(Box<LifeGraph>),
}

pub struct Navigator {
    terminal: Terminal<TerminalFile, TerminalDirectory>,
    pub settings: NavigatorSettings,
    graph_mode: GraphMode,
    graph: Graph,
}

impl Default for Navigator {
    fn default() -> Self {
        Self::new()
    }
}

impl Navigator {
    pub fn new() -> Self {
        let mut terminal = Terminal::<TerminalFile, TerminalDirectory>::default();
        terminal.style = TERMINAL_STYLE;
        Self {
            terminal,
            settings: NavigatorSettings::default(),
            graph_mode: GraphMode::Triangulation,
            graph: Graph::Triangulation(Box::default()),
        }
    }
}

impl eframe::App for Navigator {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.set_visuals(egui::Visuals::dark());
        egui::Panel::top("settings_panel")
            .frame(self.settings.top_panel_frame)
            .show(ui, |ui: &mut egui::Ui| {
                ui.horizontal(|ui: &mut egui::Ui| {
                    let visuals = &mut ui.style_mut().visuals;

                    visuals.selection.bg_fill = self.settings.active_tab_bg;
                    visuals.widgets.inactive.weak_bg_fill = self.settings.inactive_tab_bg;
                    visuals.widgets.hovered.weak_bg_fill = self.settings.inactive_tab_bg;

                    let mut new_mode: GraphMode = self.graph_mode.clone();

                    let tab_text = |text: &str, is_active: bool| {
                        let text_color = if is_active {
                            self.settings.active_tab_text
                        } else {
                            self.settings.inactive_tab_text
                        };
                        egui::RichText::new(text).color(text_color).strong()
                    };

                    ui.selectable_value(
                        &mut new_mode,
                        GraphMode::Triangulation,
                        tab_text("Triangulation", self.graph_mode == GraphMode::Triangulation),
                    );
                    ui.selectable_value(
                        &mut new_mode,
                        GraphMode::Boids,
                        tab_text("Boids", self.graph_mode == GraphMode::Boids),
                    );
                    ui.selectable_value(
                        &mut new_mode,
                        GraphMode::Life,
                        tab_text("Game of Life", self.graph_mode == GraphMode::Life),
                    );

                    if new_mode != self.graph_mode {
                        self.graph_mode = new_mode.clone();
                        self.graph = match new_mode {
                            GraphMode::Triangulation => Graph::Triangulation(Box::default()),
                            GraphMode::Boids => Graph::Boids(Box::default()),
                            GraphMode::Life => Graph::Life(Box::default()),
                        };
                    }
                });
            });

        egui::Panel::bottom("terminal_panel")
            .frame(self.settings.terminal_frame)
            .min_size(MIN_TERMINAL_SIZE)
            .resizable(true)
            .show(ui, |ui: &mut egui::Ui| self.terminal.ui(ui));

        egui::CentralPanel::default()
            .frame(self.settings.graph_outer_frame)
            .show(ui, |ui: &mut egui::Ui| {
                self.settings
                    .graph_inner_frame
                    .show(ui, |ui| match &mut self.graph {
                        Graph::Triangulation(bg) => bg.ui(ui, frame),
                        Graph::Boids(bg) => bg.ui(ui, frame),
                        Graph::Life(bg) => bg.ui(ui, frame),
                    });
            });
    }

    fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match &mut self.graph {
            Graph::Triangulation(bg) => bg.logic(ctx, frame),
            Graph::Boids(bg) => bg.logic(ctx, frame),
            Graph::Life(bg) => bg.logic(ctx, frame),
        }
    }
}

/*

TODO: after first experiment is completed add to this navigator the project ui and overlay, then
make the navigator assign ids to the overlays and let the app choose which overlays correspond
to each of the tab enums and then make it interactable as to which one is clicked on and etc

*/
