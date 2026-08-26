use crate::style::{FaceStyle, LineStyle, PointStyle};
use glam::Vec2;
use kira::sound::static_sound::StaticSoundData;

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Point(Vec2, PointStyle),
    Line(Vec2, Vec2, LineStyle),
    Polygon(Vec<Vec2>, FaceStyle),
}

impl Shape {
    pub fn translate(&mut self, delta: Vec2) {
        match self {
            Self::Point(position, ..) => {
                *position += delta;
            }
            Self::Line(a, b, ..) => {
                *a += delta;
                *b += delta;
            }
            Self::Polygon(vertices, ..) => {
                for vertex in vertices.iter_mut() {
                    *vertex += delta;
                }
            }
        }
    }

    pub fn is_around(&self, origin: Vec2, radius: f32) -> bool {
        fn line_distance(p: Vec2, a: &Vec2, b: &Vec2) -> f32 {
            let ab: Vec2 = b - a;
            if ab.length() == 0.0 {
                p.distance(*a)
            } else {
                p.distance(a + ab * ((p - a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0))
            }
        }

        match self {
            Self::Point(position, ..) => origin.distance(*position) <= radius,
            Self::Line(a, b, ..) => line_distance(origin, a, b) <= radius,
            Self::Polygon(vertices, ..) => match vertices.len() {
                0 => false,
                1 => origin.distance(vertices[0]) <= radius,
                2 => line_distance(origin, &vertices[0], &vertices[1]) <= radius,
                _ => {
                    let mut is_inside: bool = false;
                    let mut j: usize = vertices.len() - 1;

                    for i in 0..vertices.len() {
                        let vi: Vec2 = vertices[i];
                        let vj: Vec2 = vertices[j];

                        if line_distance(origin, &vj, &vi) <= radius {
                            return true;
                        }

                        if (vi.y > origin.y) != (vj.y > origin.y) {
                            let intersect_x =
                                (vj.x - vi.x) * (origin.y - vi.y) / (vj.y - vi.y) + vi.x;
                            if origin.x < intersect_x {
                                is_inside = !is_inside;
                            }
                        }

                        j = i;
                    }

                    is_inside
                }
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wall {
    pub shape: Shape,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Receiver {
    pub shape: Shape,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Emitter {
    pub shape: Shape,
    pub sound_data: StaticSoundData,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SceneObject {
    Wall(Box<Wall>),
    Receiver(Box<Receiver>),
    Emitter(Box<Emitter>),
}

impl SceneObject {
    pub fn shape(&self) -> &Shape {
        match self {
            Self::Wall(w) => &w.shape,
            Self::Receiver(r) => &r.shape,
            Self::Emitter(e) => &e.shape,
        }
    }

    pub fn mut_shape(&mut self) -> &mut Shape {
        match self {
            Self::Wall(w) => &mut w.shape,
            Self::Receiver(r) => &mut r.shape,
            Self::Emitter(e) => &mut e.shape,
        }
    }
}
