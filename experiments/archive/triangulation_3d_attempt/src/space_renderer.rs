#[derive(Debug, Clone, PartialEq)]
pub enum RenderPrimitive {
    Face {
        pts: [egui::Pos2; 3],
        depth: f32,
        face_color: egui::Color32,
        stroke: egui::Stroke,
    },
    Edge {
        pts: [egui::Pos2; 2],
        depth: f32,
        stroke: egui::Stroke,
    },
    Point {
        point: egui::Pos2,
        depth: f32,
        radius: f32,
        color: egui::Color32,
    },
}

impl RenderPrimitive {
    pub fn depth(&self) -> f32 {
        match self {
            Self::Face { depth, .. } => *depth,
            Self::Edge { depth, .. } => *depth + 0.01,
            Self::Point { depth, .. } => *depth + 0.02,
        }
    }
}

// Note: intuition tells me this is probably counter intuitive to regular graphics adn using a gpu
//and windower may be prefered however to demo low spec visuals this is usable, in addition
//optimizations can be made to this renderer

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SpaceRenderer {
    pub primitives_buffer: Vec<RenderPrimitive>,
}

impl SpaceRenderer {
    pub fn draw_ui(&mut self, ui: &mut egui::Ui) {
        let painter: &egui::Painter = ui.painter();

        self.primitives_buffer.sort_by(|a, b| {
            b.depth()
                .partial_cmp(&a.depth())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for prim in self.primitives_buffer.iter() {
            match prim {
                RenderPrimitive::Face {
                    pts,
                    face_color,
                    stroke,
                    ..
                } => {
                    painter.add(egui::Shape::convex_polygon(
                        pts.to_vec(),
                        *face_color,
                        *stroke,
                    ));
                }
                RenderPrimitive::Edge { pts, stroke, .. } => {
                    painter.line_segment(*pts, *stroke);
                }
                RenderPrimitive::Point {
                    point,
                    radius,
                    color,
                    ..
                } => {
                    painter.circle_filled(*point, *radius, *color);
                }
            }
        }

        self.primitives_buffer.clear();
    }
}
