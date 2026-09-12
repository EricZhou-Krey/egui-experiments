use crate::graph::GraphUpdate;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BoidGraph {
    state: usize,
}

impl BoidGraph {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<GraphUpdate> {
        ui.heading("boid_graph");
        //TODO currently
        // Basic implementation, order or operations ->
        // 1. camera -> projecting into 3d
        // 2. scene -> starting with a spacital box without obstacles
        //      -> Holds Quadtree<Boid>
        //      -> Objects
        //      -> Bounds (Rect (f32, f32, f32, f32))
        // 3. octree optimized binary partition of points
        //      -> just n depth bins that is queried for adjacent, move down in tree structure
        // 4. boids representation
        //      -> Boid(Vec2) -> can be directly placed into the octree struct
        None
    }

    pub fn logic(&mut self, _ctx: &egui::Context) {}
}
