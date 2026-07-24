use glam::{I8Vec2, Vec2};
use shared_view::Viewable;

#[derive(Default)]
pub struct TriangulationGraph {
    init_flag: bool, // Memory alignment issues :(
    n_points: usize,
    points: Vec<Vec2>,
    point_speeds: Vec<Vec2>,
    edges: Vec<I8Vec2>,
}

impl TriangulationGraph {
    pub fn re_initialize(&mut self, n_points: usize, point_speed: f32, dimensions: Vec2) {
        self.n_points = n_points;
        self.points = (0..self.n_points)
            .map(|_| Vec2 {
                x: rand::random::<f32>() * dimensions.x,
                y: rand::random::<f32>() * dimensions.y,
            })
            .collect();

        self.point_speeds = (0..self.n_points)
            .map(|_| Vec2 {
                x: (rand::random::<f32>() - 0.5) * point_speed,
                y: (rand::random::<f32>() - 0.5) * point_speed,
            })
            .collect();

        self.update_edges();
    }

    fn update_points(&mut self) {
        for i in 0..self.points.len() {
            self.points[i] += self.point_speeds[i];
        }
    }

    fn update_edges(&mut self) {}
}

impl Viewable for TriangulationGraph {
    fn title(&self) -> &str {
        "Detri Graph"
    }

    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        if !self.init_flag {
            self.re_initialize(
                100,
                0.1,
                Vec2 {
                    x: ui.available_width(),
                    y: ui.available_height(),
                },
            );
            self.init_flag = true;
        }

        self.update_points();
        self.update_edges();

        let debug_info: String = format!("points: {:?}, \n edges: {:?}", self.points, self.edges);
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
