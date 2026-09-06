use eframe::egui::{Color32, CornerRadius, Frame, Margin, Stroke};
use eframe::epaint::Shadow;
use terminal::TerminalStyle;

use crate::style::{FaceStyle, GraphStyle, LineStyle, PointStyle};

pub const TRIANGULATION_GRAPH_STYLE: GraphStyle = GraphStyle {
    point: PointStyle {
        radius: 8.0,
        color: Color32::WHITE,
    },
    point_heavy: PointStyle {
        radius: 8.0,
        color: Color32::GRAY,
    },
    point_light: PointStyle {
        radius: 3.0,
        color: Color32::GRAY,
    },
    line: LineStyle {
        stroke: Stroke {
            width: 1.5,
            color: Color32::WHITE,
        },
    },
    line_heavy: LineStyle {
        stroke: Stroke {
            width: 3.0,
            color: Color32::WHITE,
        },
    },
    line_light: LineStyle {
        stroke: Stroke {
            width: 1.0,
            color: Color32::GRAY,
        },
    },
    face: FaceStyle {
        fill_color: Color32::BLACK,
        border_stroke: Stroke::NONE,
    },
    face_heavy: FaceStyle {
        fill_color: Color32::from_gray(40),
        border_stroke: Stroke::NONE,
    },
    face_light: FaceStyle {
        fill_color: Color32::from_gray(20),
        border_stroke: Stroke::NONE,
    },
};

pub const ACTIVE_TAB_BG: Color32 = Color32::WHITE;
pub const ACTIVE_TAB_TEXT: Color32 = Color32::BLACK;

pub const INACTIVE_TAB_BG: Color32 = Color32::from_gray(150);
pub const INACTIVE_TAB_TEXT: Color32 = Color32::WHITE;

pub const MIN_TERMINAL_SIZE: f32 = 8.0;

pub const TOP_PANEL_FRAME: Frame = Frame {
    inner_margin: Margin {
        left: 8,
        right: 8,
        top: 4,
        bottom: 4,
    },
    outer_margin: Margin {
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
    },
    corner_radius: CornerRadius {
        nw: 0,
        ne: 0,
        sw: 0,
        se: 0,
    },
    shadow: Shadow::NONE,
    fill: Color32::BLACK,
    stroke: Stroke {
        width: 1.0,
        color: Color32::WHITE,
    },
};

pub const GRAPH_OUTER_FRAME: Frame = Frame {
    inner_margin: Margin {
        left: 3,
        right: 3,
        top: 3,
        bottom: 3,
    },
    outer_margin: Margin {
        left: 8,
        right: 8,
        top: 8,
        bottom: 8,
    },
    corner_radius: CornerRadius {
        nw: 0,
        ne: 0,
        sw: 0,
        se: 0,
    },
    shadow: Shadow {
        offset: [4, 4],
        blur: 0,
        spread: 0,
        color: Color32::from_gray(80),
    },
    fill: Color32::BLACK,
    stroke: Stroke {
        width: 1.0,
        color: Color32::WHITE,
    },
};

pub const GRAPH_INNER_FRAME: Frame = Frame {
    inner_margin: Margin {
        left: 10,
        right: 10,
        top: 10,
        bottom: 10,
    },
    outer_margin: Margin {
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
    },
    corner_radius: CornerRadius {
        nw: 0,
        ne: 0,
        sw: 0,
        se: 0,
    },
    shadow: Shadow::NONE,
    fill: Color32::BLACK,
    stroke: Stroke::NONE,
};

pub const TERMINAL_FRAME: Frame = Frame {
    inner_margin: Margin {
        left: 5,
        right: 5,
        top: 5,
        bottom: 5,
    },
    outer_margin: Margin {
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
    },
    corner_radius: CornerRadius {
        nw: 0,
        ne: 0,
        sw: 0,
        se: 0,
    },
    shadow: Shadow::NONE,
    fill: Color32::BLACK,
    stroke: Stroke {
        width: 1.0,
        color: Color32::WHITE,
    },
};

pub const TERMINAL_STYLE: TerminalStyle = TerminalStyle {
    background_color: Color32::BLACK,
    background_corner_radius: 0.0,
    prompt_text_color: egui::Color32::from_rgb(80, 250, 120),
    selection_color: egui::Color32::from_rgba_premultiplied(100, 100, 100, 100),
    text_color: egui::Color32::from_rgb(200, 200, 200),
    text_style: egui::TextStyle::Monospace,
    user: "bird",
    host: "rook-os",
};
