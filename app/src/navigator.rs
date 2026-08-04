use crate::triangulation_graph::TriangulationGraph;
use shared_view::Viewable;

#[derive(Debug, Default, Clone, PartialEq)]
enum OverlayUi {
    #[default]
    Title,

    ExampleOne,
    ExampleTwo,

    VariantCount, // Dummy discrimant variable
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
            _ => {}
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct NavigatorSettings {
    mouse_interact_radius: f32,
}

impl NavigatorSettings {
    pub fn new() -> Self {
        Self {
            mouse_interact_radius: 100.0,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Navigator {
    settings: NavigatorSettings,

    triangulation_graph: TriangulationGraph,

    overlay: OverlayUi,
    active_nodes: Vec<usize>,
    max_index: usize,
    active_index: usize,
}

impl Navigator {
    pub fn new() -> Self {
        Self {
            settings: NavigatorSettings::new(),
            triangulation_graph: TriangulationGraph::new(),
            ..Default::default()
        }
    }

    fn assign_active_nodes(&mut self) {
        let n_active: usize =
            (OverlayUi::VariantCount as usize).min(self.triangulation_graph.settings.n_points);

        self.max_index = self.triangulation_graph.settings.n_points;
        self.active_nodes =
            rand::seq::index::sample(&mut rand::rng(), self.max_index, n_active).into_vec();
    }

    fn handle_input(&mut self, ui: &mut egui::Ui) {
        let mut mouse_click_position: Option<egui::Pos2> = None;
        ui.input(|i| {
            if i.pointer.primary_pressed() {
                mouse_click_position = i.pointer.interact_pos();
            }
        });

        if let Some(click_pos) = mouse_click_position {
            let mut selected_node: Option<usize> = None;

            let mut closest_so_far: Option<f32> = None;
            for (i, node) in self.active_nodes.iter().enumerate() {
                let node_position: glam::Vec2 =
                    self.triangulation_graph.screen_points[*node].0.into();
                let click_position: glam::Vec2 = (click_pos.x, click_pos.y).into();

                let click_distance: f32 = (node_position - click_position).length();
                if click_distance < closest_so_far.unwrap_or(self.settings.mouse_interact_radius) {
                    closest_so_far = Some(click_distance);
                    selected_node = Some(i);
                }
            }

            self.active_index = selected_node.unwrap_or(0);
        }
    }
}

impl Viewable for Navigator {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        // Inefficient checking, could refactor
        if self.triangulation_graph.settings.n_points != self.max_index {
            self.assign_active_nodes()
        }

        self.handle_input(ui);
        self.triangulation_graph.draw_ui(ui);

        // START TEMP
        // Refactor
        //      - Decouple painter and triangulation graph, send requests to draw ui at the very
        //      end after all calculations and such
        //      Render struct stores and is passed as &mut to projections and draw_render is called
        //      once at end of structures, used for graphs and such

        let painter: &egui::Painter = ui.painter();

        for node in self.active_nodes.iter() {
            painter.circle_filled(
                self.triangulation_graph.screen_points[*node].0.into(),
                10.0,
                egui::Color32::WHITE,
            );
        }

        painter.circle_filled(
            self.triangulation_graph.screen_points[self.active_nodes[self.active_index]]
                .0
                .into(),
            20.0,
            egui::Color32::GREEN,
        );

        // END TEMP

        self.overlay = match self.active_index {
            1 => OverlayUi::ExampleOne,
            2 => OverlayUi::ExampleTwo,
            _ => OverlayUi::Title,
        };

        self.overlay.draw_ui(ui);
    }
}
