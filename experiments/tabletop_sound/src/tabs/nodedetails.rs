use crate::scene::scene_object::{SceneObject, Shape};
use crate::settings::style::{FaceStyle, LineStyle, PointStyle};
use crate::state::TTSState;
use glam::Vec2;

use crate::terminal::command::{
    CommandFormatter, ModifyAction, ModifySceneObjectArgs, ModifySceneObjectCommand,
};
use crate::terminal::TTSTerminalExt;
use terminal::app::AppCommand;

pub fn nodedetails_title(_state: &mut TTSState) -> egui::WidgetText {
    "Node Details".into()
}

pub fn nodedetails_ui(state: &mut TTSState, ui: &mut egui::Ui) {
    let mut commands_to_dispatch = Vec::new();

    if let Some(object_key) = state.map.selected_object_key {
        let filename = state
            .terminal
            .get_object_filename(object_key)
            .unwrap_or_default();
        let target_path = format!("scene/{}", filename);

        let (type_name, description, duration) = {
            if let Some(object) = state.scene.objects.get(object_key) {
                match object {
                    SceneObject::Wall(_) => ("Wall", "An acoustic barrier.", None),
                    SceneObject::Receiver(_) => ("Receiver", "An acoustic listener.", None),
                    SceneObject::Emitter(_) => ("Emitter", "A sound source.", Some(10.0)),
                }
            } else {
                ui.centered_and_justified(|ui: &mut egui::Ui| {
                    ui.heading("Object Not Found");
                });
                return;
            }
        };

        ui.heading(format!("Selected: {}_(ID: {:?})", type_name, object_key));
        ui.separator();

        ui.label(format!("Description: {}", description));
        if let Some(dur) = duration {
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Sound Data:");
                ui.label(format!("{:.2} seconds", dur));
            });
        }

        ui.separator();
        ui.heading("Shape & Position");

        if let Some(object) = state.scene.objects.get(object_key) {
            shape_ui(ui, object.shape(), &target_path, &mut commands_to_dispatch);
        }
    } else {
        ui.centered_and_justified(|ui: &mut egui::Ui| {
            ui.heading("No Selection");
        });
    }

    for cmd in commands_to_dispatch {
        ModifySceneObjectCommand::execute_app(state, &cmd);
    }
}

fn shape_ui(ui: &mut egui::Ui, shape: &Shape, target_path: &str, commands: &mut Vec<Vec<String>>) {
    match shape {
        Shape::Point(position, point_style) => {
            ui.label("Type: Point");
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Position:");
                let mut px = position.x;
                let mut py = position.y;

                let x_changed = ui
                    .add(egui::DragValue::new(&mut px).prefix("X: "))
                    .changed();
                let y_changed = ui
                    .add(egui::DragValue::new(&mut py).prefix("Y: "))
                    .changed();

                if x_changed || y_changed {
                    commands.push(
                        ModifySceneObjectArgs {
                            target_path: target_path.to_string(),
                            action: ModifyAction::SetPos(Vec2::new(px, py)),
                        }
                        .format_args(),
                    );
                }
            });

            ui.separator();
            ui.collapsing("Point Style", |ui: &mut egui::Ui| {
                if let Some(new_style) = point_style_ui(ui, point_style) {
                    commands.push(
                        ModifySceneObjectArgs {
                            target_path: target_path.to_string(),
                            action: ModifyAction::SetPointStyle(new_style),
                        }
                        .format_args(),
                    );
                }
            });
        }

        Shape::Line(a, b, line_style, opt_point_style) => {
            ui.label("Type: Line");

            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Point A:");
                let mut ax = a.x;
                let mut ay = a.y;
                if ui
                    .add(egui::DragValue::new(&mut ax).prefix("X: "))
                    .changed()
                    || ui
                        .add(egui::DragValue::new(&mut ay).prefix("Y: "))
                        .changed()
                {
                    commands.push(
                        ModifySceneObjectArgs {
                            target_path: target_path.to_string(),
                            action: ModifyAction::SetLineA(Vec2::new(ax, ay)),
                        }
                        .format_args(),
                    );
                }
            });

            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Point B:");
                let mut bx = b.x;
                let mut by = b.y;
                if ui
                    .add(egui::DragValue::new(&mut bx).prefix("X: "))
                    .changed()
                    || ui
                        .add(egui::DragValue::new(&mut by).prefix("Y: "))
                        .changed()
                {
                    commands.push(
                        ModifySceneObjectArgs {
                            target_path: target_path.to_string(),
                            action: ModifyAction::SetLineB(Vec2::new(bx, by)),
                        }
                        .format_args(),
                    );
                }
            });

            ui.separator();
            ui.collapsing("Line Style", |ui: &mut egui::Ui| {
                if let Some(new_style) = line_style_ui(ui, line_style) {
                    commands.push(
                        ModifySceneObjectArgs {
                            target_path: target_path.to_string(),
                            action: ModifyAction::SetLineStyle(new_style),
                        }
                        .format_args(),
                    );
                }
            });

            let mut has_endpoints: bool = opt_point_style.is_some();
            if ui
                .checkbox(&mut has_endpoints, "Enable Endpoint Styles")
                .changed()
            {
                commands.push(
                    ModifySceneObjectArgs {
                        target_path: target_path.to_string(),
                        action: ModifyAction::EnableEndpoints(has_endpoints),
                    }
                    .format_args(),
                );
            }

            if let Some(point_style) = opt_point_style {
                ui.collapsing("Endpoint Point Style", |ui: &mut egui::Ui| {
                    if let Some(new_style) = point_style_ui(ui, point_style) {
                        commands.push(
                            ModifySceneObjectArgs {
                                target_path: target_path.to_string(),
                                action: ModifyAction::SetEndpointStyle(new_style),
                            }
                            .format_args(),
                        );
                    }
                });
            }
        }

        Shape::Polygon(vertices, face_style, opt_line_style, opt_point_style) => {
            ui.label(format!("Type: Polygon ({} vertices)", vertices.len()));

            ui.collapsing("Vertices", |ui: &mut egui::Ui| {
                egui::ScrollArea::vertical()
                    .id_salt("polygon_vertices")
                    .max_height(150.0_f32)
                    .show(ui, |ui: &mut egui::Ui| {
                        let mut index_to_remove: Option<usize> = None;

                        for (i, vertex) in vertices.iter().enumerate() {
                            ui.horizontal(|ui: &mut egui::Ui| {
                                ui.label(format!("#{}", i));
                                let mut vx = vertex.x;
                                let mut vy = vertex.y;

                                if ui
                                    .add(egui::DragValue::new(&mut vx).prefix("X: "))
                                    .changed()
                                    || ui
                                        .add(egui::DragValue::new(&mut vy).prefix("Y: "))
                                        .changed()
                                {
                                    commands.push(
                                        ModifySceneObjectArgs {
                                            target_path: target_path.to_string(),
                                            action: ModifyAction::SetVertex(i, Vec2::new(vx, vy)),
                                        }
                                        .format_args(),
                                    );
                                }

                                if ui.button("X").clicked() {
                                    index_to_remove = Some(i);
                                }
                            });
                        }

                        if let Some(i) = index_to_remove {
                            commands.push(
                                ModifySceneObjectArgs {
                                    target_path: target_path.to_string(),
                                    action: ModifyAction::RemoveVertex(i),
                                }
                                .format_args(),
                            );
                        }

                        if ui.button("+ Add Vertex").clicked() {
                            let new_vertex = vertices.last().copied().unwrap_or(Vec2::ZERO);
                            commands.push(
                                ModifySceneObjectArgs {
                                    target_path: target_path.to_string(),
                                    action: ModifyAction::AddVertex(new_vertex),
                                }
                                .format_args(),
                            );
                        }
                    });
            });

            ui.separator();
            ui.collapsing("Face Style", |ui: &mut egui::Ui| {
                if let Some(new_style) = face_style_ui(ui, face_style) {
                    commands.push(
                        ModifySceneObjectArgs {
                            target_path: target_path.to_string(),
                            action: ModifyAction::SetFaceStyle(new_style),
                        }
                        .format_args(),
                    );
                }
            });

            let mut has_border: bool = opt_line_style.is_some();
            if ui.checkbox(&mut has_border, "Enable Border").changed() {
                commands.push(
                    ModifySceneObjectArgs {
                        target_path: target_path.to_string(),
                        action: ModifyAction::EnableBorder(has_border),
                    }
                    .format_args(),
                );
            }

            if let Some(line_style) = opt_line_style {
                ui.collapsing("Border Line Style", |ui: &mut egui::Ui| {
                    if let Some(new_style) = line_style_ui(ui, line_style) {
                        commands.push(
                            ModifySceneObjectArgs {
                                target_path: target_path.to_string(),
                                action: ModifyAction::SetBorderStyle(new_style),
                            }
                            .format_args(),
                        );
                    }
                });
            }

            let mut has_vertex_points: bool = opt_point_style.is_some();
            if ui
                .checkbox(&mut has_vertex_points, "Enable Vertex Styles")
                .changed()
            {
                commands.push(
                    ModifySceneObjectArgs {
                        target_path: target_path.to_string(),
                        action: ModifyAction::EnableVertexPoints(has_vertex_points),
                    }
                    .format_args(),
                );
            }

            if let Some(point_style) = opt_point_style {
                ui.collapsing("Vertex Point Style", |ui: &mut egui::Ui| {
                    if let Some(new_style) = point_style_ui(ui, point_style) {
                        commands.push(
                            ModifySceneObjectArgs {
                                target_path: target_path.to_string(),
                                action: ModifyAction::SetVertexStyle(new_style),
                            }
                            .format_args(),
                        );
                    }
                });
            }
        }
    }
}

fn point_style_ui(ui: &mut egui::Ui, style: &PointStyle) -> Option<PointStyle> {
    let mut new_style = style.clone();
    let mut changed = false;
    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Radius:");
        changed |= ui
            .add(
                egui::DragValue::new(&mut new_style.radius)
                    .speed(0.1_f32)
                    .range(0.0_f32..=1000.0_f32),
            )
            .changed();
    });
    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Color:");
        changed |= ui.color_edit_button_srgba(&mut new_style.color).changed();
    });
    if changed {
        Some(new_style)
    } else {
        None
    }
}

fn line_style_ui(ui: &mut egui::Ui, style: &LineStyle) -> Option<LineStyle> {
    let mut new_style = style.clone();
    let mut changed = false;
    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Width:");
        changed |= ui
            .add(
                egui::DragValue::new(&mut new_style.width)
                    .speed(0.1_f32)
                    .range(0.0_f32..=1000.0_f32),
            )
            .changed();
    });
    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Color:");
        changed |= ui.color_edit_button_srgba(&mut new_style.color).changed();
    });
    if changed {
        Some(new_style)
    } else {
        None
    }
}

fn face_style_ui(ui: &mut egui::Ui, style: &FaceStyle) -> Option<FaceStyle> {
    let mut new_style = style.clone();
    let mut changed = false;
    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Fill Color:");
        changed |= ui
            .color_edit_button_srgba(&mut new_style.fill_color)
            .changed();
    });
    if changed {
        Some(new_style)
    } else {
        None
    }
}
