use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::scene_object::SceneObject;
use crate::{
    scene::Scene,
    style_sheet::{LEFT_PANEL_WIDTH, TOP_LEFT_PANEL_HEIGHT, TOP_RIGHT_PANEL_HEIGHT},
    tab::Tab,
    tabs::{mapview::MapState, terminal::TTSTerminal},
};
use egui_dock::{DockState, NodeIndex, TabViewer, Tree};
use glam::Vec2;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TTSSettings {}

pub struct TTSState {
    scene: Rc<RefCell<Scene>>,
    pub map: MapState,
    pub terminal: TTSTerminal,
    pub settings: TTSSettings,
}

impl Default for TTSState {
    fn default() -> Self {
        let scene: Rc<RefCell<Scene>> = Rc::new(RefCell::new(Scene::default()));

        Self {
            scene: scene.clone(),
            map: MapState::default(),
            terminal: TTSTerminal::new(scene),
            settings: TTSSettings::default(),
        }
    }
}

impl Deref for TTSState {
    type Target = TTSSettings;
    fn deref(&self) -> &Self::Target {
        &self.settings
    }
}

impl DerefMut for TTSState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.settings
    }
}

impl TabViewer for TTSState {
    type Tab = Tab;
    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(tab)
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.title(self)
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.ui(self, ui);
    }
}

impl TTSState {
    pub fn find_scene_object_index_around(&self, position: Vec2, radius: f32) -> Option<usize> {
        self.scene
            .borrow()
            .objects
            .iter()
            .rposition(|object: &Rc<RefCell<SceneObject>>| -> bool {
                object.borrow().shape().is_around(position, radius)
            })
    }

    pub fn add_scene_object(&mut self, object: SceneObject) -> usize {
        let rc_object: Rc<RefCell<SceneObject>> = Rc::new(RefCell::new(object));

        let index: usize = {
            let mut scene: std::cell::RefMut<Scene> = self.scene.borrow_mut();
            let idx: usize = scene.objects.len();
            scene.objects.push(rc_object.clone());
            idx
        };

        self.terminal.add_scene_object(index, rc_object);

        index
    }

    pub fn remove_scene_object(&mut self, index: usize) -> Option<Rc<RefCell<SceneObject>>> {
        let removed_object: Option<Rc<RefCell<SceneObject>>> = {
            let mut scene: std::cell::RefMut<Scene> = self.scene.borrow_mut();
            if index < scene.objects.len() {
                Some(scene.objects.remove(index))
            } else {
                None
            }
        };

        if removed_object.is_some() {
            let new_total: usize = self.scene.borrow().objects.len();
            self.terminal.remove_scene_object(index, new_total);
        }

        removed_object
    }

    pub fn scene_objects(&self) -> Ref<'_, Vec<Rc<RefCell<SceneObject>>>> {
        Ref::map(self.scene.borrow(), |scene| &scene.objects)
    }

    pub fn scene_objects_mut(&self) -> RefMut<'_, Vec<Rc<RefCell<SceneObject>>>> {
        RefMut::map(self.scene.borrow_mut(), |scene| &mut scene.objects)
    }

    pub fn scene_object(&self, index: usize) -> Option<Rc<RefCell<SceneObject>>> {
        self.scene.borrow().objects.get(index).cloned()
    }

    pub fn default_dock() -> DockState<Tab> {
        let mut dock: DockState<Tab> = DockState::new(vec![Tab::MapView]);

        let surface: &mut Tree<Tab> = dock.main_surface_mut();

        let root_panel: NodeIndex = NodeIndex::root();

        let [right_panel, left_panel]: [NodeIndex; 2] =
            surface.split_left(root_panel, LEFT_PANEL_WIDTH, vec![Tab::NodeDetails]);

        let [_map_panel, _console_sound_panel]: [NodeIndex; 2] = surface.split_below(
            right_panel,
            TOP_RIGHT_PANEL_HEIGHT,
            vec![Tab::Terminal, Tab::SoundView],
        );

        let [_play_node_details_panel, _node_tree_panel]: [NodeIndex; 2] = surface.split_below(
            left_panel,
            TOP_LEFT_PANEL_HEIGHT,
            vec![Tab::NodeTree, Tab::PlayControls],
        );

        dock
    }

    pub fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
}
