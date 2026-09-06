use egui::Frame;

use crate::settings::{
    logic_sheet::{
        EXAMPLE_N_TRIANGULATION_INTERACTABLE, INTERACTION_RADIUS, N_TRIANGULATION_VERTICES,
        TRIANGULATION_MESH_ZOOM, TRIANGULATION_VERTEX_SPEED,
    },
    style_sheet::GRAPH_FRAME,
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
    pub graph_frame: Frame,
}

impl Default for NavigatorSettings {
    fn default() -> Self {
        Self {
            graph_frame: GRAPH_FRAME,
        }
    }
}
