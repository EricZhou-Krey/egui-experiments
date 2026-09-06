use eframe::egui;
use terminal::{
    file_system::{TerminalDirectory, TerminalFile},
    Terminal,
};

use crate::{
    boids::graph::BoidGraph,
    life::graph::LifeGraph,
    settings::{style_sheet::MIN_TERMINAL_SIZE, NavigatorSettings},
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
        Self {
            terminal: Terminal::<TerminalFile, TerminalDirectory>::default(),
            settings: NavigatorSettings::default(),
            graph_mode: GraphMode::Triangulation,
            graph: Graph::Triangulation(Box::default()),
        }
    }
}

impl eframe::App for Navigator {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::Panel::top("settings_panel").show(ui, |ui: &mut egui::Ui| {
            ui.horizontal(|ui: &mut egui::Ui| {
                let mut new_mode: GraphMode = self.graph_mode.clone();

                // Can you change the styling for these selectable values
                ui.selectable_value(&mut new_mode, GraphMode::Triangulation, "Triangulation");
                ui.selectable_value(&mut new_mode, GraphMode::Boids, "Boids");
                ui.selectable_value(&mut new_mode, GraphMode::Life, "Game of Life");

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

        // TODO: I want to add a floating overlay on the background depending on the selected node
        // of the background determined by internal calls -> display title and etc -> do not
        // comment on this rn

        // What different styling options are there here?
        egui::CentralPanel::default()
            .frame(self.settings.graph_frame)
            .show(ui, |ui: &mut egui::Ui| match &mut self.graph {
                Graph::Triangulation(bg) => bg.ui(ui, frame),
                Graph::Boids(bg) => bg.ui(ui, frame),
                Graph::Life(bg) => bg.ui(ui, frame),
            });

        // How many styling options
        egui::Panel::bottom("terminal_panel")
            .min_size(MIN_TERMINAL_SIZE)
            .show(ui, |ui: &mut egui::Ui| self.terminal.ui(ui));
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

Need to refactor the interface between triangulation and stuff

*/
