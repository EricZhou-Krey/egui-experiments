use glam::Vec2;

pub struct SoundRay {
    pub origin: Vec2,
    pub direction: Vec2,
    pub distance_travelled: f32,
    pub energy: f32,
}

impl SoundRay {
    pub fn new(origin: Vec2, direction: Vec2) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
            distance_travelled: 0.0,
            energy: 1.0,
        }
    }

    pub fn intersect_segment(&self, a: Vec2, b: Vec2) -> Option<(Vec2, Vec2, f32)> {
        let v1 = self.origin - a;
        let v2 = b - a;
        let v3 = Vec2::new(-self.direction.y, self.direction.x);

        let dot = v2.dot(v3);
        if dot.abs() < 0.00001 {
            return None;
        }

        let t1 = (v2.x * v1.y - v2.y * v1.x) / dot;
        let t2 = self.direction.dot(v1) / dot;

        if t1 >= 0.0 && (0.0..=1.0).contains(&t2) {
            let point = self.origin + self.direction * t1;
            let normal = Vec2::new(-v2.y, v2.x).normalize();
            Some((point, normal, t1))
        } else {
            None
        }
    }
}
