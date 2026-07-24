use glam::Vec2;
use shared_view::Viewable;

#[derive(Default)]
pub struct TriangulationGraph {
    init_flag: bool, // Memory alignment issues :(
    n_points: usize,
    points: Vec<Vec2>,
}

impl Viewable for TriangulationGraph {
    fn title(&self) -> &str {
        "Detri Graph"
    }

    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        if !self.init_flag {
            self.n_points = 100;
            self.points = (0..self.n_points)
                .map(|_| Vec2 {
                    x: rand::random::<f32>() * ui.available_width(),
                    y: rand::random::<f32>() * ui.available_height(),
                })
                .collect();
        }
        let debug_info: String = format!("{:?}", self.points);
        ui.heading(debug_info.as_str());
    }
}

#[derive(Default)]
pub struct Navigator {
    triangulation_graph: TriangulationGraph,
}

impl Viewable for Navigator {
    fn title(&self) -> &str {
        "Navigator"
    }

    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        self.triangulation_graph.draw_ui(ui);
    }

    fn is_closeable(&self) -> bool {
        false
    }
}
