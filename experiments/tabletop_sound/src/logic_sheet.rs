pub const MAP_INTERACTION_RADIUS: f32 = 20.0;
pub const MAP_ZOOM_SENSITIVITY: f32 = 0.01;
pub const MAP_ZOOM_LIMIT: f32 = 0.1;
pub const MAP_BASE_ZOOM: f32 = 1.0;

pub fn generate_sample_transmitter_sound() -> kira::sound::static_sound::StaticSoundData {
    let sample_rate: u32 = 44100;
    let duration_seconds: f32 = 1.0;
    let num_samples: usize = (sample_rate as f32 * duration_seconds) as usize;

    let mut frames: Vec<kira::Frame> = Vec::with_capacity(num_samples);
    let frequency: f32 = 440.0;

    for i in 0..num_samples {
        let t: f32 = i as f32 / sample_rate as f32;
        let sample: f32 = (t * frequency * 2.0 * std::f32::consts::PI).sin() * 0.5;

        frames.push(kira::Frame::from_mono(sample));
    }

    kira::sound::static_sound::StaticSoundData {
        sample_rate,
        slice: Some((0, frames.len())),
        frames: std::sync::Arc::from(frames),
        settings: kira::sound::static_sound::StaticSoundSettings::new(),
    }
}
