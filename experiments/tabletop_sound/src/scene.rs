use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
enum SceneObject {
    Wall,
    Reciever,
    Transmitter,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SceneSettings;

#[derive(Default, Debug, Clone)]
pub struct Scene {
    objects: HashSet<SceneObject>,
    settings: SceneSettings,
}
