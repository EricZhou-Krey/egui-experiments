use crate::triangulation_graph::TriangulationGraph;
use shared_view::Viewable;

#[derive(Default)]
enum NavigatorOverlay {
    #[default]
    Navigator,

    ExampleNodeOne,
    ExampleNodeTwo,
}

impl NavigatorOverlay {
    fn draw_overlay(&self, ui: &mut egui::Ui) {
        match self {
            Self::Navigator => {
                egui::Frame::new()
                    .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE))
                    .fill(egui::Color32::BLACK)
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        ui.label("This is the navigator overlay");
                    });
            }
            Self::ExampleNodeOne => {
                egui::Frame::new()
                    .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE))
                    .fill(egui::Color32::BLACK)
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        ui.label("This is the example one overlay");
                    });
            }
            Self::ExampleNodeTwo => {
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
    overlay: NavigatorOverlay,
}

// PLAN:
// Selectable point on triangulation triangulation_graph
//      - Set velocity to 0
//      - Present preview at top left or right of the Project
//      - Title changes to description of the Project
//          - First example will be the tabletop sound Project

impl Navigator {
    fn handle_input(&mut self, ui: &mut egui::Ui) {
        let mut debug_value: egui::Pos2 = egui::Pos2::default();
        ui.input(|i| {
            for key in &i.keys_down {
                match key {
                    egui::Key::Num1 => self.overlay = NavigatorOverlay::Navigator,
                    egui::Key::Num2 => self.overlay = NavigatorOverlay::ExampleNodeOne,
                    egui::Key::Num3 => self.overlay = NavigatorOverlay::ExampleNodeTwo,
                    _ => {}
                }
            }

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
        self.overlay.draw_overlay(ui);
    }
}
