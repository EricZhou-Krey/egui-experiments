use egui::{Color32, Stroke};

#[derive(Debug, Default, Clone)]
pub struct PointStyle {
    pub radius: f32,
    pub color: Color32,
}

#[derive(Debug, Default, Clone)]
pub struct LineStyle {
    pub stroke: Stroke,
}

#[derive(Debug, Default, Clone)]
pub struct FaceStyle {
    pub fill_color: Color32,
    pub border_stroke: Stroke,
}

#[derive(Debug, Default, Clone)]
pub struct GraphStyle {
    pub point_light: PointStyle,
    pub point: PointStyle,
    pub point_heavy: PointStyle,
    pub line_light: LineStyle,
    pub line: LineStyle,
    pub line_heavy: LineStyle,
    pub face_light: FaceStyle,
    pub face: FaceStyle,
    pub face_heavy: FaceStyle,
}
