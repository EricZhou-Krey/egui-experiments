use egui::{
    Align, Color32, FontFamily, FontId, Layout, Painter, Pos2, Rect, Response, RichText, Sense,
    UiBuilder,
};
use shared_view::Viewable;

// TODO: Change ui system:
// - Make nodes
// Reconfigure the title to only be displayed at top left and cover with bounding box and not
// display preview unless click occurs,
// sense drag and align preview according to the placement on the click on a node, title disappears
// then and etc

pub struct Navigator {}

impl Default for Navigator {
    fn default() -> Self {
        Self {}
    }
}

impl Navigator {
    fn draw_node_graph(&mut self, ui: &mut egui::Ui) {
        let (response, painter): (Response, Painter) =
            ui.allocate_painter(ui.available_size(), Sense::hover());
    }
}

impl Viewable for Navigator {
    fn title(&self) -> &str {
        "📊 Navigator"
    }

    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        self.draw_node_graph(ui);
    }

    fn is_closeable(&self) -> bool {
        false
    }
}
