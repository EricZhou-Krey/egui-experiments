use egui::{Color32, Stroke};

use crate::style_sheet::{
    MAP_ADDRECEIVER_ICON, MAP_ADDTRANSMITTER_ICON, MAP_ADDWALL_ICON, MAP_BACKGROUND_COLOR,
    MAP_MOVE_ICON, MAP_PAN_ICON, MAP_RECEIVER_COLOR, MAP_RECEIVER_RADIUS, MAP_REMOVE_ICON,
    MAP_SELECT_ICON, MAP_TRANSMITTER_COLOR, MAP_TRANSMITTER_RADIUS, MAP_WALL_BORDER_COLOR,
    MAP_WALL_BORDER_STROKE_WIDTH, MAP_WALL_FILL_COLOR, MAP_WALL_VERTEX_COLOR,
    MAP_WALL_VERTEX_RADIUS, MAP_ZOOM_ICON,
};

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

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MapIcons {
    pub select_tool: &'static str,
    pub pan_tool: &'static str,
    pub remove_tool: &'static str,
    pub add_wall_tool: &'static str,
    pub add_receiver_tool: &'static str,
    pub add_transmitter_tool: &'static str,
    pub move_tool: &'static str,
    pub zoom_tool: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapStyle {
    pub receiver: PointStyle,
    pub transmitter: PointStyle,
    pub wall_vertex: PointStyle,
    pub wall_face: FaceStyle,
    pub background: BackgroundStyle,
    pub icons: MapIcons,
}

impl Default for MapStyle {
    fn default() -> Self {
        Self {
            receiver: PointStyle {
                radius: MAP_RECEIVER_RADIUS,
                color: MAP_RECEIVER_COLOR,
            },
            transmitter: PointStyle {
                radius: MAP_TRANSMITTER_RADIUS,
                color: MAP_TRANSMITTER_COLOR,
            },
            wall_vertex: PointStyle {
                radius: MAP_WALL_VERTEX_RADIUS,
                color: MAP_WALL_VERTEX_COLOR,
            },
            wall_face: FaceStyle {
                fill_color: MAP_WALL_FILL_COLOR,
                border_stroke: Stroke::new(MAP_WALL_BORDER_STROKE_WIDTH, MAP_WALL_BORDER_COLOR),
            },
            background: BackgroundStyle {
                color: MAP_BACKGROUND_COLOR,
            },
            icons: MapIcons {
                select_tool: MAP_SELECT_ICON,
                pan_tool: MAP_PAN_ICON,
                remove_tool: MAP_REMOVE_ICON,
                add_wall_tool: MAP_ADDWALL_ICON,
                add_receiver_tool: MAP_ADDRECEIVER_ICON,
                add_transmitter_tool: MAP_ADDTRANSMITTER_ICON,
                move_tool: MAP_MOVE_ICON,
                zoom_tool: MAP_ZOOM_ICON,
            },
        }
    }
}
