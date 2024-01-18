use raudio_synth::gen::{sine_wave_generator, square_wave_generator, sawtooth_wave_generator, triangle_wave_generator, WaveformGenerator};
use raudio_synth::sequence::{allocate_buffers, merge_buffers, write_sequence_to_file};
use raudio_synth::freq_forms::{sine as freq_sine, square as freq_square, sawtooth as freq_sawtooth, triangle as freq_triangle, render as freq_render};
use raudio_synth::time_forms::{sine as time_sine, sawtooth as time_sawtooth, triangle as time_triangle, render_test as time_render_test};
use raudio_synth::synth_config::SynthConfig;
use std::fs;

#[test]
fn test_waveform_generation_and_writing() {
    let config = SynthConfig::new(44100, 20.0, 20000.0, 1.0, 0.0, 0.0, 1.0);
    let frequencies = [440.0, 880.0, 1760.0];
    let duration_in_seconds = 2;
    let num_samples = (config.sample_rate as f64 * duration_in_seconds as f64) as usize;

    for &freq in &frequencies {
        let mut sine_gen = sine_wave_generator(&config, freq as f32);
        let mut square_gen = square_wave_generator(&config, freq as f32);
        let mut sawtooth_gen = sawtooth_wave_generator(&config, freq as f32);
        let mut triangle_gen = triangle_wave_generator(&config, freq as f32);

        let mut sine_samples = Vec::with_capacity(num_samples);
        let mut square_samples = Vec::with_capacity(num_samples);
        let mut sawtooth_samples = Vec::with_capacity(num_samples);
        let mut triangle_samples = Vec::with_capacity(num_samples);

        for _ in 0..num_samples {
            sine_samples.push(sine_gen.next_sample() as f32);
            square_samples.push(square_gen.next_sample() as f32);
            sawtooth_samples.push(sawtooth_gen.next_sample() as f32);
            triangle_samples.push(triangle_gen.next_sample() as f32);
        }

        write_sequence_to_file(&config, &sine_samples, &format!("sine_{}.wav", freq));
        write_sequence_to_file(&config, &square_samples, &format!("square_{}.wav", freq));
        write_sequence_to_file(&config, &sawtooth_samples, &format!("sawtooth_{}.wav", freq));
        write_sequence_to_file(&config, &triangle_samples, &format!("triangle_{}.wav", freq));

        assert!(fs::metadata(format!("sine_{}.wav", freq)).is_ok());
        assert!(fs::metadata(format!("square_{}.wav", freq)).is_ok());
        assert!(fs::metadata(format!("sawtooth_{}.wav", freq)).is_ok());
        assert!(fs::metadata(format!("triangle_{}.wav", freq)).is_ok());
    }
}
