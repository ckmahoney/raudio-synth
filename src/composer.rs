use crate::waveforms::{sawtooth, triangle, square};
use rand::{Rng, thread_rng};

pub fn compose_sequence(duration_secs: f32, sample_rate: u32) -> Vec<f32> {
    let mut sequence = Vec::new();
    let mut rng = thread_rng();

    let samples_per_waveform = sample_rate as usize; // 1 second per waveform
    let total_waveforms = (duration_secs * sample_rate as f32) as usize / samples_per_waveform;

    for _ in 0..total_waveforms {
        let freq = rng.gen_range(220.0..880.0); // Random frequency between A3 and A5
        let waveform_type = rng.gen_range(0..3); // Choose between 0 (sawtooth), 1 (triangle), and 2 (square)

        for t in 0..samples_per_waveform {
            let t_sec = t as f32 / sample_rate as f32; // Time in seconds
            let sample = match waveform_type {
                0 => sawtooth(t_sec, freq),
                1 => triangle(t_sec, freq),
                _ => square(t_sec, freq),
            };
            sequence.push((sample * 0.5).clamp(-1.0, 1.0));
        }
    }

    sequence
}
