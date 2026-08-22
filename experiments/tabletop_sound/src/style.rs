use egui::{Color32, Stroke};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PointStyle {
    pub radius: f32,
    pub color: Color32,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct LineStyle {
    pub stroke: Stroke,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct FaceStyle {
    pub fill_color: Color32,
    pub border_stroke: Stroke,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BackgroundStyle {
    pub color: Color32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapStyle {
    pub receiver: PointStyle,
    pub transmitter: PointStyle,
    pub line: LineStyle,
    pub face: FaceStyle,
    pub background: BackgroundStyle,
}

impl Default for MapStyle {
    fn default() -> Self {
        Self {
            receiver: PointStyle {
                radius: 5.0,
                color: Color32::from_rgb(50, 150, 255),
            },
            transmitter: PointStyle {
                radius: 6.0,
                color: Color32::from_rgb(255, 80, 80),
            },
            line: LineStyle {
                stroke: Stroke::new(2.0, Color32::from_rgb(180, 180, 180)),
            },
            face: FaceStyle {
                fill_color: Color32::from_rgba_unmultiplied(100, 150, 200, 30),
                border_stroke: Stroke::new(
                    1.0,
                    Color32::from_rgba_unmultiplied(100, 150, 200, 100),
                ),
            },
            background: BackgroundStyle {
                color: Color32::from_rgb(30, 30, 30),
            },
        }
    }
}
