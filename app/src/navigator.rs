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

pub struct Navigator {
    title_font: FontFamily,
    title_font_size: f32,

    preview_title_split: f32,

    preview_size: f32,
    preview_aspect_ratio: f32,

    top_grid_split: f32,

    cell_size: f32,
    grid_width: u32,
    grid_height: u32,

    spotlight_radius: f32,
    spotlight_center: Pos2,
    spotlight_color: Color32,
}

impl Default for Navigator {
    fn default() -> Self {
        Self {
            title_font: FontFamily::Monospace,
            title_font_size: 150.,

            preview_title_split: 0.4,

            preview_size: 200.,
            preview_aspect_ratio: 16. / 9.,

            cell_size: 100.0,
            grid_width: 8,
            grid_height: 8,

            top_grid_split: 0.4,

            spotlight_radius: 8.0,
            spotlight_center: Pos2 { x: -1., y: -1. },
            spotlight_color: Color32::CYAN,
        }
    }
}

impl Viewable for Navigator {
    fn title(&self) -> &str {
        "📊 Navigator"
    }

    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        let (top_rect, _): (Rect, Rect) = ui
            .content_rect()
            .split_top_bottom_at_fraction(self.preview_title_split);

        let (preview_rect, title_rect): (Rect, Rect) =
            top_rect.split_left_right_at_fraction(self.top_grid_split);

        ui.scope_builder(UiBuilder::new().max_rect(ui.max_rect()), |grid_ui| {
            let (response, painter): (Response, Painter) =
                grid_ui.allocate_painter(grid_ui.available_size(), Sense::hover());
            let screen_center = grid_ui.content_rect().center();

            // Math constants for 2:1 Isometric Projection (approx 30 degrees)
            let cos30 = std::f32::consts::FRAC_PI_6.cos();
            let sin30 = std::f32::consts::FRAC_PI_6.sin();

            // Projection helper: World (x, y) -> Screen (x, y)
            let project = |x: f32, y: f32| -> egui::Pos2 {
                let iso_x = (x - y) * cos30;
                let iso_y = (x + y) * sin30;
                screen_center + egui::vec2(iso_x * self.cell_size, iso_y * self.cell_size)
            };

            // Inverse Projection helper: Screen (x, y) -> World (x, y)
            let unproject = |screen_pos: egui::Pos2| -> egui::Pos2 {
                let offset = screen_pos - screen_center;
                let dx = offset.x / (cos30 * self.cell_size);
                let dy = offset.y / (sin30 * self.cell_size);

                let world_x = (dx + dy) / 2.0;
                let world_y = (dy - dx) / 2.0;
                egui::pos2(world_x, world_y)
            };

            // 2. Determine spotlight position (default to 0,0, but follow mouse if hovering)
            let mut spotlight_world = egui::pos2(0.0, 0.0);
            if let Some(mouse_pos) = response.hover_pos() {
                spotlight_world = unproject(mouse_pos);
            }

            // 3. Draw the grid
            // Calculate half extents to center the grid based on width and height
            let half_w = (self.grid_width / 2) as i32;
            let half_h = (self.grid_height / 2) as i32;

            for x in -half_w..=half_w {
                for y in -half_h..=half_h {
                    let x_f = x as f32;
                    let y_f = y as f32;

                    // Calculate distance from spotlight center in WORLD space
                    // This creates a circular spotlight on the plane (which looks elliptical on screen)
                    let dist = (x_f - spotlight_world.x).hypot(y_f - spotlight_world.y);

                    // If it's outside the radius, skip drawing entirely to save performance
                    if dist > self.spotlight_radius {
                        continue;
                    }

                    // Calculate intensity (1.0 at center, 0.0 at edge)
                    let intensity = 1.0 - (dist / self.spotlight_radius).powf(1.5); // powf adds a smoother curve

                    if intensity > 0.0 {
                        // Apply intensity to the spotlight_color field
                        let base_color = self.spotlight_color;
                        let color = egui::Color32::from_rgba_premultiplied(
                            (base_color.r() as f32 * intensity) as u8,
                            (base_color.g() as f32 * intensity) as u8,
                            (base_color.b() as f32 * intensity) as u8,
                            (base_color.a() as f32 * intensity) as u8,
                        );
                        let stroke = egui::Stroke::new(2.0, color);

                        // Calculate vertices for the local cell
                        let p_base = project(x_f, y_f);
                        let p_right = project(x_f + 1.0, y_f); // Segment moving along X
                        let p_down = project(x_f, y_f + 1.0); // Segment moving along Y

                        // Draw X axis segment
                        painter.line_segment([p_base, p_right], stroke);
                        // Draw Y axis segment
                        painter.line_segment([p_base, p_down], stroke);
                    }
                }
            }
        });

        let floating_frame = egui::Frame::window(ui.style())
            .inner_margin(16.0)
            .outer_margin(16.0);

        ui.scope_builder(
            UiBuilder::new()
                .max_rect(title_rect)
                .layout(Layout::top_down(Align::Max)),
            |title_ui| {
                floating_frame.show(title_ui, |frame_ui| {
                    frame_ui.heading(
                        RichText::new("TITLE")
                            .font(FontId::new(self.title_font_size, self.title_font.clone()))
                            .strong(),
                    );
                });
            },
        );

        ui.scope_builder(
            UiBuilder::new()
                .max_rect(preview_rect)
                .layout(Layout::top_down(Align::Min)),
            |preview_ui| {
                floating_frame.show(preview_ui, |frame_ui| {
                    frame_ui.heading("Placeholder for video player");
                });
            },
        );
    }

    fn is_closeable(&self) -> bool {
        false
    }
}
