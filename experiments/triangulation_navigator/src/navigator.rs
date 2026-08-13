use crate::triangulation_background::TriangulationBackground;
use shared_view::viewable::Viewable;

#[derive(Debug, Default, Clone)]
pub struct Navigator {
    triangulation_background: TriangulationBackground,
}

impl Navigator {
    pub fn new() -> Self {
        Self {
            triangulation_background: TriangulationBackground::new(),
        }
    }
}

impl Viewable for Navigator {
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        self.triangulation_background.draw_ui(ui);
    }
}
