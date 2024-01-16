use crate::waveforms::{sawtooth, triangle, square};
use rand::{Rng, thread_rng};

#[allow(dead_code)]
pub fn compose_sequence(duration_secs: f32, sample_rate: u32) -> Vec<f32> {
    let mut sequence = Vec::new();
    let mut rng = thread_rng();
    let dur = 1;    
    let samples_per_waveform = dur * sample_rate; 
    let total_waveforms = (duration_secs * sample_rate as f32) / samples_per_waveform as f32;

    for _ in 0..total_waveforms as u32 {
        let freq = rng.gen_range(220.0..880.0); // Random frequency between A3 and A5
        let waveform_type: i32 = rng.gen_range(0..3); // Choose between 0 (sawtooth), 1 (triangle), and 2 (square)

        for t in 0..samples_per_waveform {
            let t_sec = t / sample_rate; // Time in seconds
            let bias = 0.5;
            let sample = match waveform_type {
                0 => sawtooth(t_sec, freq, bias, sample_rate),
                1 => triangle(t_sec, freq, bias, sample_rate),
                _ => square(t_sec, freq, bias, sample_rate),
            };
            sequence.push((sample * 0.5).clamp(-1.0, 1.0));
        }
    }

    sequence
}