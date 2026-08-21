#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub enum Tab {
    #[default]
    Empty,
    MapView,
    NodeDetails,
    Console,
    NodeTree,
    SoundView,
    PlayControls,
}
