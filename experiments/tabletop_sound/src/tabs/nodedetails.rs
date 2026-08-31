use crate::scene_object::{SceneObject, Shape};
use crate::state::TTSState;
use crate::style::{FaceStyle, LineStyle, PointStyle};
use std::cell::RefMut;

pub fn nodedetails_title(_state: &mut TTSState) -> egui::WidgetText {
    "Node Details".into()
}

pub fn nodedetails_ui(state: &mut TTSState, ui: &mut egui::Ui) {
    if let Some(object_index) = state.map.selected_object_index {
        if let Some(scene_object_rc) = state.scene_object(object_index) {
            let mut scene_object: RefMut<SceneObject> = scene_object_rc.borrow_mut();

            ui.heading(std::format!(
                "Selected: {}_{}.obj",
                scene_object.type_name(),
                object_index
            ));
            ui.separator();

            match &mut *scene_object {
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
                        ui.label(std::format!("{:.2} seconds", duration));
                    });
                }
            }

            ui.separator();
            ui.heading("Shape & Position");

            let shape: &mut Shape = scene_object.mut_shape();
            shape_ui(ui, shape);
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

fn shape_ui(ui: &mut egui::Ui, shape: &mut Shape) {
    match shape {
        Shape::Point(position, point_style) => {
            ui.label("Type: Point");
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Position:");
                ui.add(egui::DragValue::new(&mut position.x).prefix("X: "));
                ui.add(egui::DragValue::new(&mut position.y).prefix("Y: "));
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
                ui.add(egui::DragValue::new(&mut a.x).prefix("X: "));
                ui.add(egui::DragValue::new(&mut a.y).prefix("Y: "));
            });

            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Point B:");
                ui.add(egui::DragValue::new(&mut b.x).prefix("X: "));
                ui.add(egui::DragValue::new(&mut b.y).prefix("Y: "));
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
            ui.label(std::format!("Type: Polygon ({} vertices)", vertices.len()));

            ui.collapsing("Vertices", |ui: &mut egui::Ui| {
                egui::ScrollArea::vertical()
                    .id_salt("polygon_vertices")
                    .max_height(150.0_f32)
                    .show(ui, |ui: &mut egui::Ui| {
                        let mut index_to_remove: Option<usize> = None;

                        for (i, vertex) in vertices.iter_mut().enumerate() {
                            ui.horizontal(|ui: &mut egui::Ui| {
                                ui.label(std::format!("#{}", i));
                                ui.add(egui::DragValue::new(&mut vertex.x).prefix("X: "));
                                ui.add(egui::DragValue::new(&mut vertex.y).prefix("Y: "));

                                if ui.button("X").clicked() {
                                    index_to_remove = Some(i);
                                }
                            });
                        }

                        if let Some(i) = index_to_remove {
                            vertices.remove(i);
                        }

                        if ui.button("+ Add Vertex").clicked() {
                            let new_vertex: glam::Vec2 =
                                vertices.last().copied().unwrap_or(glam::Vec2::ZERO);
                            vertices.push(new_vertex);
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
