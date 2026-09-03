use crate::scene::scene_object::{SceneObject, Shape};
use crate::scene::SpatialNode;
use crate::settings::style::{FaceStyle, LineStyle, PointStyle};
use crate::state::TTSState;
use glam::Vec2;

pub fn nodedetails_title(_state: &mut TTSState) -> egui::WidgetText {
    "Node Details".into()
}

pub fn nodedetails_ui(state: &mut TTSState, ui: &mut egui::Ui) {
    if let Some(object_key) = state.map.selected_object_key {
        let mut editor = state.edit_scene();

        if let Some(object) = editor.scene.objects.get_mut(object_key) {
            let (old_min, old_max) = object.shape().logical_bounds();

            let type_name = match object {
                SceneObject::Wall(_) => "Wall",
                SceneObject::Receiver(_) => "Receiver",
                SceneObject::Emitter(_) => "Emitter",
            };

            ui.heading(format!("Selected: {}_{:?}", type_name, object_key));
            ui.separator();

            match object {
                SceneObject::Wall(_) => {
                    ui.label("Description: An acoustic barrier.");
                }
                SceneObject::Receiver(_) => {
                    ui.label("Description: An acoustic listener.");
                }
                SceneObject::Emitter(emitter) => {
                    ui.label("Description: A sound source.");
                    ui.horizontal(|ui: &mut egui::Ui| {
                        ui.label("Sound Data:");
                        let duration: f32 = emitter.sound_data.duration().as_secs_f32();
                        ui.label(format!("{:.2} seconds", duration));
                    });
                }
            }

            ui.separator();
            ui.heading("Shape & Position");

            let shape = object.mut_shape();
            let position_changed = shape_ui(ui, shape);

            if position_changed {
                let (new_min, new_max) = shape.logical_bounds();

                editor.scene.quadtree.remove(&SpatialNode {
                    key: object_key,
                    min: old_min,
                    max: old_max,
                });

                editor.scene.quadtree.insert(SpatialNode {
                    key: object_key,
                    min: new_min,
                    max: new_max,
                });
            }
        } else {
            ui.centered_and_justified(|ui: &mut egui::Ui| {
                ui.heading("Object Not Found");
            });
        }
    } else {
        ui.centered_and_justified(|ui: &mut egui::Ui| {
            ui.heading("No Selection");
        });
    }
}

fn shape_ui(ui: &mut egui::Ui, shape: &mut Shape) -> bool {
    let mut changed = false;

    match shape {
        Shape::Point(position, point_style) => {
            ui.label("Type: Point");
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Position:");
                changed |= ui
                    .add(egui::DragValue::new(&mut position.x).prefix("X: "))
                    .changed();
                changed |= ui
                    .add(egui::DragValue::new(&mut position.y).prefix("Y: "))
                    .changed();
            });

            ui.separator();
            ui.collapsing("Point Style", |ui: &mut egui::Ui| {
                point_style_ui(ui, point_style);
            });
        }

        Shape::Line(a, b, line_style, opt_point_style) => {
            ui.label("Type: Line");

            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Point A:");
                changed |= ui
                    .add(egui::DragValue::new(&mut a.x).prefix("X: "))
                    .changed();
                changed |= ui
                    .add(egui::DragValue::new(&mut a.y).prefix("Y: "))
                    .changed();
            });

            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Point B:");
                changed |= ui
                    .add(egui::DragValue::new(&mut b.x).prefix("X: "))
                    .changed();
                changed |= ui
                    .add(egui::DragValue::new(&mut b.y).prefix("Y: "))
                    .changed();
            });

            ui.separator();
            ui.collapsing("Line Style", |ui: &mut egui::Ui| {
                line_style_ui(ui, line_style);
            });

            let mut has_endpoints: bool = opt_point_style.is_some();
            if ui
                .checkbox(&mut has_endpoints, "Enable Endpoint Styles")
                .changed()
            {
                if has_endpoints {
                    *opt_point_style = Some(PointStyle::default());
                } else {
                    *opt_point_style = None;
                }
            }

            if let Some(point_style) = opt_point_style {
                ui.collapsing("Endpoint Point Style", |ui: &mut egui::Ui| {
                    point_style_ui(ui, point_style);
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

                        for (i, vertex) in vertices.iter_mut().enumerate() {
                            ui.horizontal(|ui: &mut egui::Ui| {
                                ui.label(format!("#{}", i));
                                changed |= ui
                                    .add(egui::DragValue::new(&mut vertex.x).prefix("X: "))
                                    .changed();
                                changed |= ui
                                    .add(egui::DragValue::new(&mut vertex.y).prefix("Y: "))
                                    .changed();

                                if ui.button("X").clicked() {
                                    index_to_remove = Some(i);
                                    changed = true;
                                }
                            });
                        }

                        if let Some(i) = index_to_remove {
                            vertices.remove(i);
                        }

                        if ui.button("+ Add Vertex").clicked() {
                            let new_vertex = vertices.last().copied().unwrap_or(Vec2::ZERO);
                            vertices.push(new_vertex);
                            changed = true;
                        }
                    });
            });

            ui.separator();
            ui.collapsing("Face Style", |ui: &mut egui::Ui| {
                face_style_ui(ui, face_style);
            });

            let mut has_border: bool = opt_line_style.is_some();
            if ui.checkbox(&mut has_border, "Enable Border").changed() {
                if has_border {
                    *opt_line_style = Some(LineStyle::default());
                } else {
                    *opt_line_style = None;
                }
            }

            if let Some(line_style) = opt_line_style {
                ui.collapsing("Border Line Style", |ui: &mut egui::Ui| {
                    line_style_ui(ui, line_style);
                });
            }

            let mut has_vertex_points: bool = opt_point_style.is_some();
            if ui
                .checkbox(&mut has_vertex_points, "Enable Vertex Styles")
                .changed()
            {
                if has_vertex_points {
                    *opt_point_style = Some(PointStyle::default());
                } else {
                    *opt_point_style = None;
                }
            }

            if let Some(point_style) = opt_point_style {
                ui.collapsing("Vertex Point Style", |ui: &mut egui::Ui| {
                    point_style_ui(ui, point_style);
                });
            }
        }
    }

    changed
}

fn point_style_ui(ui: &mut egui::Ui, style: &mut PointStyle) {
    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Radius:");
        ui.add(
            egui::DragValue::new(&mut style.radius)
                .speed(0.1_f32)
                .range(0.0_f32..=1000.0_f32),
        );
    });
    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Color:");
        ui.color_edit_button_srgba(&mut style.color);
    });
}

fn line_style_ui(ui: &mut egui::Ui, style: &mut LineStyle) {
    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Width:");
        ui.add(
            egui::DragValue::new(&mut style.width)
                .speed(0.1_f32)
                .range(0.0_f32..=1000.0_f32),
        );
    });
    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Color:");
        ui.color_edit_button_srgba(&mut style.color);
    });
}

fn face_style_ui(ui: &mut egui::Ui, style: &mut FaceStyle) {
    ui.horizontal(|ui: &mut egui::Ui| {
        ui.label("Fill Color:");
        ui.color_edit_button_srgba(&mut style.fill_color);
    });
}
