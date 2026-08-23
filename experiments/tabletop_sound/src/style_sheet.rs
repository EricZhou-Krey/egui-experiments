pub const LEFT_PANEL_WIDTH: f32 = 0.2;
pub const TOP_LEFT_PANEL_HEIGHT: f32 = 0.8;
pub const TOP_RIGHT_PANEL_HEIGHT: f32 = 0.8;

pub const MAP_SELECT_ICON: &str = "S";
pub const MAP_ADDWALL_ICON: &str = "A";
pub const MAP_ADDTRANSMITTER_ICON: &str = "A";
pub const MAP_ADDRECEIVER_ICON: &str = "A";
pub const MAP_REMOVE_ICON: &str = "R";
pub const MAP_PAN_ICON: &str = "P";

pub const MAP_TOOLBAR_PADDING: egui::Vec2 = egui::vec2(8.0, 8.0);
pub const MAP_TOOLBAR_MARGIN: f32 = 4.0;
pub const MAP_TOOLBAR_CORNER_RADIUS: f32 = 6.0;
pub const MAP_TOOLBAR_BUTTON_SIZE: egui::Vec2 = egui::vec2(32.0, 32.0);

pub const MAP_RECEIVER_RADIUS: f32 = 5.0;
pub const MAP_RECEIVER_COLOR: egui::Color32 = egui::Color32::from_rgb(50, 150, 255);

pub const MAP_TRANSMITTER_RADIUS: f32 = 6.0;
pub const MAP_TRANSMITTER_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 80, 80);

pub const MAP_WALL_FILL_COLOR: egui::Color32 =
    egui::Color32::from_rgba_premultiplied(100, 150, 200, 30);
pub const MAP_WALL_BORDER_STROKE_WIDTH: f32 = 1.0;
pub const MAP_WALL_BORDER_COLOR: egui::Color32 =
    egui::Color32::from_rgba_premultiplied(100, 150, 200, 100);

pub const MAP_BACKGROUND_COLOR: egui::Color32 = egui::Color32::from_rgb(30, 30, 30);
