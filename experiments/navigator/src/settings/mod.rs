use egui::{Color32, Frame};

use crate::settings::{
    logic_sheet::{
        EXAMPLE_N_TRIANGULATION_INTERACTABLE, INTERACTION_RADIUS, N_TRIANGULATION_VERTICES,
        TRIANGULATION_MESH_ZOOM, TRIANGULATION_VERTEX_SPEED,
    },
    style_sheet::{
        ACTIVE_TAB_BG, ACTIVE_TAB_TEXT, GRAPH_INNER_FRAME, GRAPH_OUTER_FRAME, INACTIVE_TAB_BG,
        INACTIVE_TAB_TEXT, TERMINAL_FRAME, TOP_PANEL_FRAME,
    },
};

pub mod logic_sheet;
pub mod style_sheet;

#[derive(Debug, Clone, PartialEq)]
pub struct InteractableTriangulationMeshSettings {
    pub n_internal_vertices: usize,
    pub vertex_speed: f32,
    pub n_interactable: usize,
    pub interaction_radius: f32,
}

impl Default for InteractableTriangulationMeshSettings {
    fn default() -> Self {
        Self {
            n_internal_vertices: N_TRIANGULATION_VERTICES,
            vertex_speed: TRIANGULATION_VERTEX_SPEED,
            n_interactable: EXAMPLE_N_TRIANGULATION_INTERACTABLE,
            interaction_radius: INTERACTION_RADIUS,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TriangulationGraphSettings {
    pub mesh_zoom: f32,
}

impl Default for TriangulationGraphSettings {
    fn default() -> Self {
        Self {
            mesh_zoom: TRIANGULATION_MESH_ZOOM,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavigatorSettings {
    pub top_panel_frame: Frame,
    pub graph_outer_frame: Frame,
    pub graph_inner_frame: Frame,
    pub terminal_frame: Frame,

    pub active_tab_bg: Color32,
    pub active_tab_text: Color32,
    pub inactive_tab_bg: Color32,
    pub inactive_tab_text: Color32,
}

impl Default for NavigatorSettings {
    fn default() -> Self {
        Self {
            top_panel_frame: TOP_PANEL_FRAME,
            graph_outer_frame: GRAPH_OUTER_FRAME,
            graph_inner_frame: GRAPH_INNER_FRAME,
            terminal_frame: TERMINAL_FRAME,

            active_tab_bg: ACTIVE_TAB_BG,
            active_tab_text: ACTIVE_TAB_TEXT,
            inactive_tab_bg: INACTIVE_TAB_BG,
            inactive_tab_text: INACTIVE_TAB_TEXT,
        }
    }
}
