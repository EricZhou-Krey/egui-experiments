use glam::Vec2;
use terminal::{
    app::AppCommand,
    file_system::{FileSystemNode, TerminalFile},
    Terminal,
};

use crate::{
    scene::{
        scene_object::{Emitter, Receiver, SceneObject, Shape, Wall},
        SpatialNode,
    },
    settings::style::{FaceStyle, LineStyle, PointStyle},
    state::TTSState,
    terminal::{
        file_system::{TTSDirectory, TTSFile},
        TTSTerminalExt,
    },
};

pub trait CommandFormatter {
    fn format_args(&self) -> Vec<String>;
}

fn push_color_args(args: &mut Vec<String>, color: egui::Color32) {
    args.push(color.r().to_string());
    args.push(color.g().to_string());
    args.push(color.b().to_string());
    args.push(color.a().to_string());
}

pub enum AddSceneObjectArgs {
    Emitter(Vec2),
    Receiver(Vec2),
    Wall(Vec<Vec2>),
}

impl CommandFormatter for AddSceneObjectArgs {
    fn format_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        match self {
            Self::Emitter(pos) => {
                args.push("emitter".to_string());
                args.push(pos.x.to_string());
                args.push(pos.y.to_string());
            }
            Self::Receiver(pos) => {
                args.push("receiver".to_string());
                args.push(pos.x.to_string());
                args.push(pos.y.to_string());
            }
            Self::Wall(vertices) => {
                args.push("wall".to_string());
                for v in vertices {
                    args.push(v.x.to_string());
                    args.push(v.y.to_string());
                }
            }
        }
        args
    }
}

pub struct RemoveSceneObjectArgs {
    pub target_path: String,
}

impl CommandFormatter for RemoveSceneObjectArgs {
    fn format_args(&self) -> Vec<String> {
        vec![self.target_path.clone()]
    }
}

pub enum ModifyAction {
    Translate(Vec2),
    SetPos(Vec2),
    SetPointStyle(PointStyle),
    SetEndpointStyle(PointStyle),
    SetVertexStyle(PointStyle),
    SetLineA(Vec2),
    SetLineB(Vec2),
    SetLineStyle(LineStyle),
    SetBorderStyle(LineStyle),
    SetFaceStyle(FaceStyle),
    SetVertex(usize, Vec2),
    AddVertex(Vec2),
    RemoveVertex(usize),
    EnableEndpoints(bool),
    EnableBorder(bool),
    EnableVertexPoints(bool),
}

pub struct ModifySceneObjectArgs {
    pub target_path: String,
    pub action: ModifyAction,
}

impl CommandFormatter for ModifySceneObjectArgs {
    fn format_args(&self) -> Vec<String> {
        let mut args = vec![self.target_path.clone()];
        match &self.action {
            ModifyAction::Translate(pos) => {
                args.push("--translate".to_string());
                args.push(pos.x.to_string());
                args.push(pos.y.to_string());
            }
            ModifyAction::SetPos(pos) => {
                args.push("--set_pos".to_string());
                args.push(pos.x.to_string());
                args.push(pos.y.to_string());
            }
            ModifyAction::SetPointStyle(style) => {
                args.push("--set_point_style".to_string());
                args.push(style.radius.to_string());
                push_color_args(&mut args, style.color);
            }
            ModifyAction::SetEndpointStyle(style) => {
                args.push("--set_endpoint_style".to_string());
                args.push(style.radius.to_string());
                push_color_args(&mut args, style.color);
            }
            ModifyAction::SetVertexStyle(style) => {
                args.push("--set_vertex_style".to_string());
                args.push(style.radius.to_string());
                push_color_args(&mut args, style.color);
            }
            ModifyAction::SetLineA(pos) => {
                args.push("--set_line_a".to_string());
                args.push(pos.x.to_string());
                args.push(pos.y.to_string());
            }
            ModifyAction::SetLineB(pos) => {
                args.push("--set_line_b".to_string());
                args.push(pos.x.to_string());
                args.push(pos.y.to_string());
            }
            ModifyAction::SetLineStyle(style) => {
                args.push("--set_line_style".to_string());
                args.push(style.width.to_string());
                push_color_args(&mut args, style.color);
            }
            ModifyAction::SetBorderStyle(style) => {
                args.push("--set_border_style".to_string());
                args.push(style.width.to_string());
                push_color_args(&mut args, style.color);
            }
            ModifyAction::SetFaceStyle(style) => {
                args.push("--set_face_style".to_string());
                push_color_args(&mut args, style.fill_color);
            }
            ModifyAction::SetVertex(idx, pos) => {
                args.push("--set_vertex".to_string());
                args.push(idx.to_string());
                args.push(pos.x.to_string());
                args.push(pos.y.to_string());
            }
            ModifyAction::AddVertex(pos) => {
                args.push("--add_vertex".to_string());
                args.push(pos.x.to_string());
                args.push(pos.y.to_string());
            }
            ModifyAction::RemoveVertex(idx) => {
                args.push("--remove_vertex".to_string());
                args.push(idx.to_string());
            }
            ModifyAction::EnableEndpoints(enable) => {
                args.push("--enable_endpoints".to_string());
                args.push(enable.to_string());
            }
            ModifyAction::EnableBorder(enable) => {
                args.push("--enable_border".to_string());
                args.push(enable.to_string());
            }
            ModifyAction::EnableVertexPoints(enable) => {
                args.push("--enable_vertex_points".to_string());
                args.push(enable.to_string());
            }
        }
        args
    }
}

pub struct TTSInfoCommand;

impl AppCommand<TTSState> for TTSInfoCommand {
    fn name() -> &'static str {
        "info"
    }

    fn execute_app(tts: &mut TTSState, args: &[String]) {
        let terminal: &mut Terminal<TTSFile, TTSDirectory> = &mut tts.terminal.base;

        if let Some(target_file) = args.first() {
            let mut file_path: Vec<String> = terminal.current_directory.clone();
            file_path.push(target_file.to_string());

            let node_option = terminal.get_node(&file_path).cloned();

            if let Some(FileSystemNode::File(file)) = node_option {
                match file {
                    TTSFile::Terminal(terminal_file) => match terminal_file {
                        TerminalFile::Text(text_file) => {
                            terminal.history.push(text_file.content);
                        }
                        TerminalFile::Binary(..) => {
                            terminal
                                .history
                                .push(format!("info: {}: cannot display binary file", target_file));
                        }
                    },
                    TTSFile::SceneObject(object_key) => {
                        terminal.history.push(format!(
                            "Successfully read SceneObject {:?} from state!",
                            object_key
                        ));
                    }
                    TTSFile::SoundData(data_key) => {
                        terminal
                            .history
                            .push(format!("info: {}: cannot display binary sound file (of ID: {:?})", target_file, data_key));
                    }
                }
            } else {
                terminal
                    .history
                    .push(format!("info: {}: No such file", target_file));
            }
        } else {
            terminal
                .history
                .push("info: missing file operand".to_string());
        }
    }
}

pub struct AddSceneObjectCommand;

impl AppCommand<TTSState> for AddSceneObjectCommand {
    fn name() -> &'static str {
        "add_scene_object"
    }

    fn execute_app(tts: &mut TTSState, args: &[String]) {
        if args.is_empty() {
            tts.terminal.base.history.push(
                "add_scene_object: missing object type (emitter, receiver, wall)".to_string(),
            );
            return;
        }

        let object_type = &args[0];
        match object_type.as_str() {
            "emitter" | "receiver" => {
                if args.len() == 3 {
                    if let (Ok(x), Ok(y)) = (args[1].parse::<f32>(), args[2].parse::<f32>()) {
                        let world_position = Vec2::new(x, y);
                        let key = if object_type == "emitter" {
                            let style = tts.map.settings.style.emitter.clone();
                            let sample_sound = crate::settings::logic_sheet::generate_sample_emitter_sound();
                            let s_key = tts.sound.sounds.insert(sample_sound);

                            let k =
                                tts.scene
                                    .objects
                                    .insert(SceneObject::Emitter(Box::new(Emitter {
                                        shape: Shape::Point(world_position, style),
                                        sound_key: Some(s_key),
                                    })));
                            tts.scene.emitter_keys.insert(k);
                            k
                        } else {
                            let style = tts.map.settings.style.receiver.clone();
                            let k = tts.scene.objects.insert(SceneObject::Receiver(Receiver {
                                shape: Shape::Point(world_position, style),
                                sound_descriptor: None,
                            }));
                            tts.scene.receiver_keys.insert(k);
                            k
                        };

                        tts.terminal.register_object(&mut tts.scene, key);
                        tts.map.selected_object_key = Some(key);
                        tts.terminal.base.history.push(format!(
                            "add_scene_object: added {} at ({}, {})",
                            object_type, x, y
                        ));
                    } else {
                        tts.terminal
                            .base
                            .history
                            .push("add_scene_object: invalid coordinates".to_string());
                    }
                } else {
                    tts.terminal
                        .base
                        .history
                        .push("add_scene_object: expected x and y coordinates".to_string());
                }
            }
            "wall" => {
                let mut vertices = Vec::new();
                let vertex_args = &args[1..];
                for chunk in vertex_args.chunks(2) {
                    if chunk.len() == 2 && let (Ok(x), Ok(y)) = (chunk[0].parse::<f32>(), chunk[1].parse::<f32>()) {
                        vertices.push(Vec2::new(x, y));
                    }
                }

                if vertices.len() >= 2 {
                    let shape = Shape::Polygon(
                        vertices,
                        tts.map.settings.style.wall_face.clone(),
                        tts.map.settings.style.wall_line.clone(),
                        tts.map.settings.style.wall_vertex.clone(),
                    );
                    let key = tts.scene.objects.insert(SceneObject::Wall(Wall { shape }));

                    if let Some(SceneObject::Wall(wall)) = tts.scene.objects.get(key) {
                        let (min, max) = wall.shape.logical_bounds();
                        tts.scene
                            .wall_quadtree
                            .insert(SpatialNode { key, min, max });
                    }

                    tts.terminal.register_object(&mut tts.scene, key);
                    tts.map.selected_object_key = Some(key);
                    tts.terminal.base.history.push(format!(
                        "add_scene_object: added wall with {} vertices",
                        vertex_args.len() / 2
                    ));
                } else {
                    tts.terminal.base.history.push(
                        "add_scene_object: wall requires at least 2 vertices (4 coordinates)"
                            .to_string(),
                    );
                }
            }
            _ => tts.terminal.base.history.push(format!(
                "add_scene_object: unknown object type '{}'",
                object_type
            )),
        }
    }
}

pub struct RemoveSceneObjectCommand;

impl AppCommand<TTSState> for RemoveSceneObjectCommand {
    fn name() -> &'static str {
        "remove_scene_object"
    }

    fn execute_app(tts: &mut TTSState, args: &[String]) {
        if let Some(target_file) = args.first() {
            let file_path: Vec<String> = target_file
                .split('/')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();

            if let Some(FileSystemNode::File(TTSFile::SceneObject(key))) =
                tts.terminal.base.get_node(&file_path).cloned()
            {
                if let Some(object) = tts.scene.objects.remove(key) {
                    tts.terminal.deregister_object(&mut tts.scene, key);

                    match object {
                        SceneObject::Wall(..) => {
                            let (min, max) = object.shape().logical_bounds();
                            let node = SpatialNode { key, min, max };
                            tts.scene.wall_quadtree.remove(&node);
                        }
                        SceneObject::Emitter(..) => {
                            tts.scene.emitter_keys.remove(&key);
                        }
                        SceneObject::Receiver(..) => {
                            tts.scene.receiver_keys.remove(&key);
                        }
                    }
                    tts.terminal.base.history.push(format!(
                        "remove_scene_object: Successfully removed object at {}",
                        target_file
                    ));
                }
            } else {
                tts.terminal.base.history.push(format!(
                    "remove_scene_object: {}: No such scene object file",
                    target_file
                ));
            }
        } else {
            tts.terminal
                .base
                .history
                .push("remove_scene_object: missing file operand".to_string());
        }
    }
}

pub struct ModifySceneObjectCommand;

impl AppCommand<TTSState> for ModifySceneObjectCommand {
    fn name() -> &'static str {
        "modify_scene_object"
    }

    fn execute_app(tts: &mut TTSState, args: &[String]) {
        if args.len() < 2 {
            tts.terminal.base.history.push("modify_scene_object: missing arguments. Usage: modify_scene_object <target_path> <action> [args...]".to_string());
            return;
        }

        let target_file = &args[0];
        let action = &args[1];

        let file_path: Vec<String> = target_file
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        if let Some(FileSystemNode::File(TTSFile::SceneObject(key))) =
            tts.terminal.base.get_node(&file_path).cloned()
        {
            let mut old_node = None;
            if let Some(SceneObject::Wall(wall)) = tts.scene.objects.get(key) {
                let (min, max) = wall.shape.logical_bounds();
                old_node = Some(SpatialNode { key, min, max });
            }
            if let Some(node) = &old_node {
                tts.scene.wall_quadtree.remove(node);
            }

            let mut modification_details = String::new();

            if let Some(object) = tts.scene.objects.get_mut(key) {
                let shape = object.mut_shape();

                match action.as_str() {
                    "--translate" => {
                        if args.len() == 4 && let (Ok(x), Ok(y)) = (args[2].parse::<f32>(), args[3].parse::<f32>()) {
                            shape.translate(Vec2::new(x, y));
                            modification_details = format!("translated by ({}, {})", x, y);
                        }
                    }
                    "--set_pos" => {
                        if let Shape::Point(pos, _) = shape {
                            let old_pos = *pos;
                            pos.x = args.get(2).and_then(|v| v.parse::<f32>().ok()).unwrap_or(pos.x);
                            pos.y = args.get(3).and_then(|v| v.parse::<f32>().ok()).unwrap_or(pos.y);
                            modification_details = format!("position changed from ({}, {}) to ({}, {})", old_pos.x, old_pos.y, pos.x, pos.y);
                        }
                    }
                    "--set_point_style" | "--set_endpoint_style" | "--set_vertex_style" => {
                        let radius = args.get(2).and_then(|v| v.parse::<f32>().ok()).unwrap_or(2.0);
                        let r = args.get(3).and_then(|v| v.parse::<u8>().ok()).unwrap_or(255);
                        let g = args.get(4).and_then(|v| v.parse::<u8>().ok()).unwrap_or(255);
                        let b = args.get(5).and_then(|v| v.parse::<u8>().ok()).unwrap_or(255);
                        let a = args.get(6).and_then(|v| v.parse::<u8>().ok()).unwrap_or(255);
                        let color = egui::Color32::from_rgba_unmultiplied(r, g, b, a);
                        let style = PointStyle { radius, color };
                        let desc = format!("(radius: {}, color: {:?})", radius, color);

                        match shape {
                            Shape::Point(_, s) if action == "--set_point_style" => {
                                *s = style;
                                modification_details = format!("point style set to {}", desc);
                            }
                            Shape::Line(_, _, _, opt_s) if action == "--set_endpoint_style" => {
                                *opt_s = Some(style);
                                modification_details = format!("endpoint style set to {}", desc);
                            }
                            Shape::Polygon(_, _, _, opt_s) if action == "--set_vertex_style" => {
                                *opt_s = Some(style);
                                modification_details = format!("vertex style set to {}", desc);
                            }
                            _ => {}
                        }
                    }
                    "--set_line_a" => {
                        if let Shape::Line(a, _, _, _) = shape {
                            let old_a = *a;
                            a.x = args.get(2).and_then(|v| v.parse::<f32>().ok()).unwrap_or(a.x);
                            a.y = args.get(3).and_then(|v| v.parse::<f32>().ok()).unwrap_or(a.y);
                            modification_details = format!("line point A changed from ({}, {}) to ({}, {})", old_a.x, old_a.y, a.x, a.y);
                        }
                    }
                    "--set_line_b" => {
                        if let Shape::Line(_, b, _, _) = shape {
                            let old_b = *b;
                            b.x = args.get(2).and_then(|v| v.parse::<f32>().ok()).unwrap_or(b.x);
                            b.y = args.get(3).and_then(|v| v.parse::<f32>().ok()).unwrap_or(b.y);
                            modification_details = format!("line point B changed from ({}, {}) to ({}, {})", old_b.x, old_b.y, b.x, b.y);
                        }
                    }
                    "--set_line_style" | "--set_border_style" => {
                        let width = args.get(2).and_then(|v| v.parse::<f32>().ok()).unwrap_or(2.0);
                        let r = args.get(3).and_then(|v| v.parse::<u8>().ok()).unwrap_or(255);
                        let g = args.get(4).and_then(|v| v.parse::<u8>().ok()).unwrap_or(255);
                        let b = args.get(5).and_then(|v| v.parse::<u8>().ok()).unwrap_or(255);
                        let a = args.get(6).and_then(|v| v.parse::<u8>().ok()).unwrap_or(255);
                        let color = egui::Color32::from_rgba_unmultiplied(r, g, b, a);
                        let style = LineStyle { width, color };
                        let desc = format!("(width: {}, color: {:?})", width, color);

                        match shape {
                            Shape::Line(_, _, s, _) if action == "--set_line_style" => {
                                *s = style;
                                modification_details = format!("line style set to {}", desc);
                            }
                            Shape::Polygon(_, _, opt_s, _) if action == "--set_border_style" => {
                                *opt_s = Some(style);
                                modification_details = format!("border style set to {}", desc);
                            }
                            _ => {}
                        }
                    }
                    "--set_face_style" => {
                        if let Shape::Polygon(_, s, _, _) = shape {
                            let r = args.get(2).and_then(|v| v.parse::<u8>().ok()).unwrap_or(255);
                            let g = args.get(3).and_then(|v| v.parse::<u8>().ok()).unwrap_or(255);
                            let b = args.get(4).and_then(|v| v.parse::<u8>().ok()).unwrap_or(255);
                            let a = args.get(5).and_then(|v| v.parse::<u8>().ok()).unwrap_or(255);
                            s.fill_color = egui::Color32::from_rgba_unmultiplied(r, g, b, a);
                            modification_details = format!("face fill color set to {:?}", s.fill_color);
                        }
                    }
                    "--set_vertex" => {
                        if let Shape::Polygon(verts, _, _, _) = shape &&
                           let Some(idx) = args.get(2).and_then(|v| v.parse::<usize>().ok()) &&
                           let Some(v) = verts.get_mut(idx) {
                            let old_v = *v;
                            v.x = args.get(3).and_then(|val| val.parse::<f32>().ok()).unwrap_or(v.x);
                            v.y = args.get(4).and_then(|val| val.parse::<f32>().ok()).unwrap_or(v.y);
                            modification_details = format!("vertex {} changed from ({}, {}) to ({}, {})", idx, old_v.x, old_v.y, v.x, v.y);
                        }
                    }
                    "--add_vertex" => {
                        if let Shape::Polygon(verts, _, _, _) = shape {
                            let x = args.get(2).and_then(|v| v.parse::<f32>().ok()).unwrap_or_default();
                            let y = args.get(3).and_then(|v| v.parse::<f32>().ok()).unwrap_or_default();
                            verts.push(Vec2::new(x, y));
                            modification_details = format!("added vertex at ({}, {}) (total vertices: {})", x, y, verts.len());
                        }
                    }
                    "--remove_vertex" => {
                        if let Shape::Polygon(verts, _, _, _) = shape &&
                           let Some(idx) = args.get(2).and_then(|v| v.parse::<usize>().ok()) &&
                           idx < verts.len() {
                            let removed = verts.remove(idx);
                            modification_details = format!("removed vertex {} at ({}, {}) (remaining: {})", idx, removed.x, removed.y, verts.len());
                        }
                    }
                    "--enable_endpoints" => {
                        if let Shape::Line(_, _, _, opt_s) = shape {
                            let enable = args.get(2).map(|s| s == "true").unwrap_or(false);
                            if enable && opt_s.is_none() {
                                *opt_s = Some(PointStyle { radius: 3.0, color: egui::Color32::WHITE });
                                modification_details = "enabled endpoints".to_string();
                            } else if !enable && opt_s.is_some() {
                                *opt_s = None;
                                modification_details = "disabled endpoints".to_string();
                            }
                        }
                    }
                    "--enable_border" => {
                        if let Shape::Polygon(_, _, opt_s, _) = shape {
                            let enable = args.get(2).map(|s| s == "true").unwrap_or(false);
                            if enable && opt_s.is_none() {
                                *opt_s = Some(LineStyle { width: 2.0, color: egui::Color32::WHITE });
                                modification_details = "enabled border".to_string();
                            } else if !enable && opt_s.is_some() {
                                *opt_s = None;
                                modification_details = "disabled border".to_string();
                            }
                        }
                    }
                    "--enable_vertex_points" => {
                        if let Shape::Polygon(_, _, _, opt_s) = shape {
                            let enable = args.get(2).map(|s| s == "true").unwrap_or(false);
                            if enable && opt_s.is_none() {
                                *opt_s = Some(PointStyle { radius: 3.0, color: egui::Color32::WHITE });
                                modification_details = "enabled vertex points".to_string();
                            } else if !enable && opt_s.is_some() {
                                *opt_s = None;
                                modification_details = "disabled vertex points".to_string();
                            }
                        }
                    }
                    _ => {
                        tts.terminal.base.history.push(format!(
                            "modify_scene_object: unknown action '{}'",
                            action
                        ));
                    }
                }
            }

            if let Some(SceneObject::Wall(wall)) = tts.scene.objects.get(key) {
                let (min, max) = wall.shape.logical_bounds();
                tts.scene
                    .wall_quadtree
                    .insert(SpatialNode { key, min, max });
            }

            if !modification_details.is_empty() {
                tts.terminal.base.history.push(format!(
                    "modify_scene_object [{}]: {}",
                    target_file, modification_details
                ));
            }
        } else {
            tts.terminal.base.history.push(format!(
                "modify_scene_object: {}: No such scene object file",
                target_file
            ));
        }
    }
}
