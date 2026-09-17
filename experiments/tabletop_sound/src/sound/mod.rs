pub mod ray;

use std::f32::consts::PI;
use crate::{
    scene::{
        scene_object::{Receiver, SceneObject, Shape},
        Scene,
    },
    settings::{logic_sheet::N_RAYS, SoundSettings},
};
use glam::Vec2;
use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend, Tween,
    effect::filter::FilterBuilder,
    sound::static_sound::{StaticSoundData, StaticSoundHandle},
    track::{TrackBuilder, TrackHandle},
};
use slotmap::{new_key_type, SlotMap};

new_key_type! { pub struct SoundKey; }

pub struct SoundState {
    pub audio_manager: AudioManager,
    pub sounds: SlotMap<SoundKey, StaticSoundData>,
    pub settings: SoundSettings,
    pub active_handles: Vec<StaticSoundHandle>,
    pub active_tracks: Vec<TrackHandle>,
}

impl Default for SoundState {
    fn default() -> Self {
        Self {
            audio_manager: AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
                .unwrap(),
            sounds: SlotMap::with_key(),
            settings: SoundSettings::default(),
            active_handles: Vec::new(),
            active_tracks: Vec::new(),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct SoundFilter {
    pub volume: f64,
    pub delay_seconds: f64,
    pub low_pass_cutoff_hz: f64,
    pub panning: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PointSound {
    pub apparent_position: Vec2,
    pub sound_key: SoundKey,
    pub filter: SoundFilter,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SoundDescriptor {
    pub paths: Vec<PointSound>,
    pub outdoor_ratio: f32,
}

impl SoundState {
    pub fn generate_scene_descriptor(
        &self,
        receiver_position: Vec2,
        scene: &Scene,
    ) -> SoundDescriptor {
        let mut paths = Vec::new();
        let mut total_escaped_rays = 0;
        let mut wall_segments = Vec::new();

        for obj in scene.objects.values() {
            if let SceneObject::Wall(wall) = obj {
                match &wall.shape {
                    Shape::Polygon(verts, ..) => {
                        for i in 0..verts.len() {
                            wall_segments.push((verts[i], verts[(i + 1) % verts.len()]));
                        }
                    }
                    Shape::Line(a, b, ..) => {
                        wall_segments.push((*a, *b));
                    }
                    _ => {}
                }
            }
        }

        let ray_intersect = |origin: Vec2, dir: Vec2, a: Vec2, b: Vec2| -> Option<(Vec2, Vec2, f32)> {
            let v1 = origin - a;
            let v2 = b - a;
            let dot = v2.dot(Vec2::new(-dir.y, dir.x));

            if dot.abs() < 1e-6 {
                return None;
            }

            let t1 = (v2.x * v1.y - v2.y * v1.x) / dot;
            let t2 = dir.dot(v1) / dot;

            if t1 >= 0.0 && (0.0..=1.0).contains(&t2) {
                let mut normal = Vec2::new(-v2.y, v2.x).normalize();
                if normal.dot(dir) > 0.0 {
                    normal = -normal;
                }
                Some((origin + dir * t1, normal, t1))
            } else {
                None
            }
        };

        for &emitter_key in &scene.emitter_keys {
            if let Some(SceneObject::Emitter(emitter)) = scene.objects.get(emitter_key) &&
               let Some(sound_key) = emitter.sound_key {
                
                let emitter_pos = emitter.shape.center();
                let direct_vec = emitter_pos - receiver_position;
                let distance = direct_vec.length();
                let direct_dir = direct_vec.normalize();
                let mut wall_intersections = 0;

                for (a, b) in &wall_segments {
                    if let Some((_, _, t)) = ray_intersect(receiver_position, direct_dir, *a, *b) && t > 0.001 && t < distance {
                        wall_intersections += 1;
                    }
                }

                paths.push(PointSound {
                    apparent_position: emitter_pos,
                    sound_key,
                    filter: SoundFilter {
                        volume: (100.0 / distance.max(1.0)).clamp(0.0, 1.0) as f64,
                        delay_seconds: (distance / 343.0) as f64,
                        low_pass_cutoff_hz: (20000.0 * (0.5_f64).powi((wall_intersections / 2) as i32)).clamp(500.0, 20000.0),
                        panning: direct_dir.x as f64,
                    },
                });

                let ray_weight = 4.0 / N_RAYS as f32;

                for i in 0..N_RAYS {
                    let mut current_origin = receiver_position;
                    let mut current_dir = Vec2::from_angle(2.0 * PI * (i as f32 / N_RAYS as f32));
                    let mut dist_traveled = 0.0;
                    let mut current_energy = 1.0;

                    for bounce in 0..3 {
                        let mut closest_hit: Option<(Vec2, Vec2, f32)> = None;

                        for (a, b) in &wall_segments {
                            if let Some((hit_point, normal, t)) = ray_intersect(current_origin, current_dir, *a, *b) && t > 0.001 {
                                if closest_hit.is_none() || t < closest_hit.unwrap().2 {
                                    closest_hit = Some((hit_point, normal, t));
                                }
                            }
                        }

                        if let Some((hit_point, normal, t_to_wall)) = closest_hit {
                            dist_traveled += t_to_wall;
                            let bounce_vec = emitter_pos - hit_point;
                            let dist_to_emitter = bounce_vec.length();
                            let bounce_dir = bounce_vec.normalize();
                            let mut obstructed = false;

                            for (a, b) in &wall_segments {
                                if let Some((_, _, t)) = ray_intersect(hit_point, bounce_dir, *a, *b) && t > 0.001 && t < dist_to_emitter {
                                    obstructed = true;
                                    break;
                                }
                            }

                            if !obstructed {
                                let total_distance = dist_traveled + dist_to_emitter;
                                let ideal_reflection = current_dir - normal * 2.0 * current_dir.dot(normal);
                                let alignment = ideal_reflection.dot(bounce_dir).max(0.0);
                                let echo_volume = current_energy * alignment * ray_weight * 100.0 / total_distance.max(1.0);

                                if echo_volume > 0.005 {
                                    paths.push(PointSound {
                                        apparent_position: hit_point,
                                        sound_key,
                                        filter: SoundFilter {
                                            volume: echo_volume.clamp(0.0, 1.0) as f64,
                                            delay_seconds: (total_distance / 343.0) as f64,
                                            low_pass_cutoff_hz: (20000.0 * (0.6_f64).powi(bounce + 1)).clamp(500.0, 20000.0),
                                            panning: current_dir.x as f64,
                                        },
                                    });
                                }
                            }

                            current_dir = current_dir - normal * 2.0 * current_dir.dot(normal);
                            current_origin = hit_point;
                            current_energy *= 0.6;
                        } else {
                            if bounce == 0 {
                                total_escaped_rays += 1;
                            }
                            break;
                        }
                    }
                }
            }
        }

        SoundDescriptor {
            paths,
            outdoor_ratio: total_escaped_rays as f32 / (N_RAYS * scene.emitter_keys.len().max(1)) as f32,
        }
    }

    pub fn play(&mut self, receiver: &Receiver) {
        self.stop();

        if let Some(descriptor) = &receiver.sound_descriptor {
            let mut valid_paths = descriptor.paths.clone();
            valid_paths.sort_by(|a, b| b.filter.volume.partial_cmp(&a.filter.volume).unwrap());

            for point_sound in valid_paths.into_iter().take(60) {
                if let Some(sound_data) = self.sounds.get(point_sound.sound_key) {
                    let sd = sound_data.clone()
                        .volume(point_sound.filter.volume as f32)
                        .panning(kira::Panning(point_sound.filter.panning as f32));

                    if point_sound.filter.low_pass_cutoff_hz < 20000.0 {
                        let mut track_builder = TrackBuilder::new();
                        track_builder = track_builder.with_effect(
                            FilterBuilder::new().cutoff(point_sound.filter.low_pass_cutoff_hz as f64),
                        );

                        if let Ok(mut track_handle) = self.audio_manager.add_sub_track(track_builder) {
                            if let Ok(handle) = track_handle.play(sd) {
                                self.active_handles.push(handle);
                                self.active_tracks.push(track_handle);
                            }
                        } else if let Ok(handle) = self.audio_manager.play(sd) {
                            self.active_handles.push(handle);
                        }
                    } else if let Ok(handle) = self.audio_manager.play(sd) {
                        self.active_handles.push(handle);
                    }
                }
            }
        }
    }

    pub fn stop(&mut self) {
        for mut handle in self.active_handles.drain(..) {
            let _ = handle.stop(Tween::default());
        }
        self.active_tracks.clear();
    }

    pub fn pause(&mut self) {
        for handle in &mut self.active_handles {
            let _ = handle.pause(Tween::default());
        }
    }

    pub fn resume(&mut self) {
        for handle in &mut self.active_handles {
            let _ = handle.resume(Tween::default());
        }
    }

    pub fn seek_to(&mut self, position: f64) {
        for handle in &mut self.active_handles {
            let _ = handle.seek_to(position);
        }
    }

    pub fn seek_by(&mut self, amount: f64) {
        for handle in &mut self.active_handles {
            let _ = handle.seek_by(amount);
        }
    }
}
