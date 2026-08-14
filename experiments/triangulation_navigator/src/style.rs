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
    pub point: PointStyle,
    pub line: LineStyle,
    pub face: FaceStyle,
}
