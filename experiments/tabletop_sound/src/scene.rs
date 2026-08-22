#[derive(Debug, Clone, PartialEq)]
pub struct Wall {
    pub verticies: Vec<[f32; 2]>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Receiver {
    pub position: [f32; 2],
}

#[derive(Debug, Clone, PartialEq)]
pub struct Transmitter {
    pub position: [f32; 2],
}

#[derive(Debug, Clone, PartialEq)]
pub enum SceneObject {
    Wall(Wall),
    Receiver(Receiver),
    Transmitter(Transmitter),
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SceneSettings;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Scene {
    pub objects: Vec<SceneObject>,
    pub settings: SceneSettings,
}
