use glam::Vec2;

#[derive(Debug, Clone, PartialEq)]
pub struct Boid {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Boid {
    pub fn new_random() -> Self {
        let rx: f32 = rand::random::<f32>();
        let ry: f32 = rand::random::<f32>();

        let vx: f32 = (rand::random::<f32>() - 0.5) * 2.0;
        let vy: f32 = (rand::random::<f32>() - 0.5) * 2.0;

        Self {
            pos: glam::vec2(rx, ry),
            vel: glam::vec2(vx, vy).normalize_or_zero() * 0.2, // Initial speed
        }
    }
}
