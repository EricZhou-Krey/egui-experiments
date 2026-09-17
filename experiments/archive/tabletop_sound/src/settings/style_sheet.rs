pub const LEFT_PANEL_WIDTH: f32 = 0.2;
pub const TOP_LEFT_PANEL_HEIGHT: f32 = 0.8;
pub const TOP_RIGHT_PANEL_HEIGHT: f32 = 0.8;

pub const MAP_SELECT_ICON: &str = "S";
pub const MAP_ZOOM_ICON: &str = "Z";
pub const MAP_MOVE_ICON: &str = "M";
pub const MAP_ADDWALL_ICON: &str = "+W";
pub const MAP_ADDEMITTER_ICON: &str = "+E";
pub const MAP_ADDRECEIVER_ICON: &str = "+R";
pub const MAP_REMOVE_ICON: &str = "-";
pub const MAP_PAN_ICON: &str = "P";

pub const MAP_GRID_WIDTH: f32 = 100.0;
pub const MAP_GRID_HEIGHT: f32 = 100.0;
pub const MAP_GRID_MIN_SCREEN_SPACING: f32 = 40.0;
pub const MAP_GRID_SCALE_FACTOR: f32 = 2.0;
pub const MAP_GRID_LINE_COLOR_MULTIPLIER: f32 = 0.4;
pub const MAP_GRID_LINE_WIDTH: f32 = 1.0;
pub const MAP_GRID_TEXT_COLOR_MULTIPLIER: f32 = 0.7;
pub const MAP_GRID_TEXT_SIZE: f32 = 12.0;
pub const MAP_GRID_TEXT_OFFSET_X: f32 = 4.0;
pub const MAP_GRID_TEXT_OFFSET_Y_X_AXIS: f32 = 4.0;
pub const MAP_GRID_TEXT_OFFSET_Y_Y_AXIS: f32 = 18.0;

pub const MAP_TOOLBAR_PADDING: egui::Vec2 = egui::vec2(8.0, 8.0);
pub const MAP_TOOLBAR_MARGIN: f32 = 4.0;
pub const MAP_TOOLBAR_CORNER_RADIUS: f32 = 6.0;
pub const MAP_TOOLBAR_BUTTON_SIZE: egui::Vec2 = egui::vec2(32.0, 32.0);

pub const MAP_RECEIVER_RADIUS: f32 = 5.0;
pub const MAP_RECEIVER_COLOR: egui::Color32 = egui::Color32::from_rgb(50, 150, 255);

pub const MAP_EMITTER_RADIUS: f32 = 6.0;
pub const MAP_EMITTER_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 80, 80);

pub const MAP_WALL_VERTEX_RADIUS: f32 = 4.0;
pub const MAP_WALL_VERTEX_COLOR: egui::Color32 =
    egui::Color32::from_rgba_premultiplied(100, 150, 250, 100);
pub const MAP_WALL_FILL_COLOR: egui::Color32 =
    egui::Color32::from_rgba_premultiplied(100, 150, 200, 30);
pub const MAP_WALL_LINE_STROKE_WIDTH: f32 = 1.0;
pub const MAP_WALL_LINE_COLOR: egui::Color32 =
    egui::Color32::from_rgba_premultiplied(100, 150, 200, 100);

pub const MAP_BACKGROUND_COLOR: egui::Color32 = egui::Color32::from_rgb(30, 30, 30);
