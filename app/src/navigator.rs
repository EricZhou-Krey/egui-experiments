use std::{cell::RefCell, ops::Deref, rc::Rc};
use crate::triangulation_graph::{TriangulationGraph, TriangulationGraphSettings, TriangulationGraphOberserver};
use shared_view::Viewable;

#[derive(Default)]
enum OverlayUi {
    #[default]
    Title,

    ExampleOne,
    ExampleTwo,
}

impl Viewable for OverlayUi {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        match self {
            Self::Title => {
                egui::Frame::new()
                    .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE))
                    .fill(egui::Color32::BLACK)
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        ui.label("This is the navigator overlay");
                    });
            }
            Self::ExampleOne => {
                egui::Frame::new()
                    .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE))
                    .fill(egui::Color32::BLACK)
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        ui.label("This is the example one overlay");
                    });
            }
            Self::ExampleTwo => {
                egui::Frame::new()
                    .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE))
                    .fill(egui::Color32::BLACK)
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        ui.label("This is the example two overlay");
                    });
            }
        }
    }
}

#[derive(Default)]
struct NavigatorInfoOverlay {
    overlay_ui: OverlayUi,
    active_indicies: Vec<usize>,
}

impl TriangulationGraphOberserver for NavigatorInfoOverlay {
    fn update(&mut self, settings: &TriangulationGraphSettings) {
        println!("got signal");
    }
}

pub struct NavigatorSettings {
    mouse_interact_radius: f32,
}

impl Default for NavigatorSettings {
    fn default() -> Self {
        Self {
            mouse_interact_radius: 10.0,
        }
    }
}

#[derive(Default)]
pub struct Navigator {
    settings: NavigatorSettings,

    triangulation_graph: TriangulationGraph,
    info_overlay: Rc<RefCell<NavigatorInfoOverlay>>,
}

impl Navigator {
    pub fn new() -> Self {
        let settings = NavigatorSettings::default();
        let mut triangulation_graph = TriangulationGraph::default();
        let info_overlay = Rc::new(RefCell::new(NavigatorInfoOverlay::default()));

        triangulation_graph.observers.push(Rc::clone(&info_overlay) as Rc<RefCell<dyn TriangulationGraphOberserver>>);
        Self { settings, triangulation_graph, info_overlay }
    }

    fn handle_input(&mut self, ui: &mut egui::Ui) {
        let mut debug_value: egui::Pos2 = egui::Pos2::default();
        ui.input(|i| {
            if i.pointer.primary_pressed() && let Some(press_origin) = i.pointer.press_origin() {
                debug_value = press_origin;
            }
        });

        ui.debug_text(format!("{:?}", debug_value));
    }
}

impl Viewable for Navigator {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        self.handle_input(ui);
        self.triangulation_graph.draw_ui(ui);
        self.info_overlay.take().overlay_ui.draw_ui(ui);
    }
}

