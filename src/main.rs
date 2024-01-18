#![allow(dead_code)]
#![allow(unused_variables)]
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use hound;

mod freq_forms;
mod time_forms;
mod synth_config;
mod wavelets;
mod sequence;
mod gen;

use synth_config::SynthConfig;
use time_forms::Ugen;
use std::collections::HashMap;
use crate::gen::{WaveformGenerator, sine_wave_generator, square_wave_generator, sawtooth_wave_generator, triangle_wave_generator};

const TEST_AUDIO_DIR: &str = "test-render";

fn test_audio_name(label:&str) -> String {
    format!("{}/{}.wav", TEST_AUDIO_DIR, label)
}

pub fn render(config: &SynthConfig, ts: Vec<u32>, sr:u32, ugen: &Ugen, freq: f32, amp: f32) -> Vec<f32> {
    let mut samples: Vec<f32> = Vec::new();
    for t in ts {
        let sample = amp * ugen(config, t, freq, Some(0.5));
        samples.push(sample);
    }
    samples
}


fn render_ugen(config: &SynthConfig, ugen: &Ugen, label: &str) -> String {
    let dur_cycles = 4;
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: config.sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Int,
    };
    let name: String = format!("{}_sample-rate_{}_channels_{}", label, spec.sample_rate, spec.channels);
    let filename = test_audio_name(&name);
    let mut writer = hound::WavWriter::create(filename.clone(), spec).unwrap();
    let mut ts: Vec<u32> = Vec::new();

    for i in 0..(dur_cycles * config.sample_rate) { 
        ts.push(i) 
    };

    let sequence = render(config, ts, config.sample_rate, ugen, 440.0, 1.0);
    for sample in sequence {
        writer.write_sample(sample).unwrap();
    }
    writer.finalize().unwrap();
    filename
}


fn test_write_time_forms(config: &SynthConfig) {
    let mut shapes_map: HashMap<String, Ugen> = HashMap::new();
    shapes_map.insert(String::from("sawtooth"), time_forms::sawtooth);
    shapes_map.insert(String::from("triangle"), time_forms::triangle);
    shapes_map.insert(String::from("sine"), time_forms::sine);

    for (name, func) in &shapes_map {
        let label = format!("time_form-{}", name);
        let filename = render_ugen(&config, func, &label);
        println!("Completed writing test waveform {}", filename);
    }
}


fn test_write_freq_forms(config: &SynthConfig) {
    let mut shapes_map: HashMap<String, Ugen> = HashMap::new();
    shapes_map.insert(String::from("sawtooth"), freq_forms::sawtooth);
    shapes_map.insert(String::from("triangle"), freq_forms::triangle);
    shapes_map.insert(String::from("sine"), freq_forms::sine);

    for (name, func) in &shapes_map {
        let label = format!("freq_form-{}", name);
        let filename = render_ugen(&config, func, &label);
        println!("Completed writing test waveform {}", filename);
    }
}

fn test_write_waveforms(config: &SynthConfig) {
    test_write_time_forms(&config);
    test_write_freq_forms(&config);
}

fn test_write_sequenced_melody(config: &SynthConfig) {
    let melody = [400.0, 600.0, 500.0, 700.0, 800.0, 600.0, 500.0, 400.0];
    let waveform_functions = vec![
        freq_forms::sawtooth,
        freq_forms::triangle,
        freq_forms::sine,
    ];

    let mut rng = thread_rng();
    let mut complete_sequence: Vec<f32> = Vec::new();

    for (index, &frequency) in melody.iter().enumerate() {
        let ugen = waveform_functions.choose(&mut rng).unwrap();
        let note_duration = index + 1;
        let mut sequence = Vec::with_capacity(note_duration as usize);
        let num_samples = (config.sample_rate as f32 * note_duration as f32 / config.cps).floor() as i32;
        for i in 0..num_samples {
            let sample = ugen(config, i as u32, frequency, None);
            sequence.push(sample);
        }

        complete_sequence.extend(sequence);
    }

    let label = "melody-test";
    write_sequence_to_file(&config, &complete_sequence, label);
}


fn test_waveform_generator(config: &SynthConfig) {
    let sample_rate = config.sample_rate as f64;
    let frequency = 400;
    let duration_in_seconds = 2;
    let num_samples = (sample_rate * duration_in_seconds as f64) as usize;

    // Generate waveforms
    let mut sine_gen = sine_wave_generator(&config, frequency as f32);
    let mut square_gen = square_wave_generator(&config, frequency as f32);
    let mut sawtooth_gen = sawtooth_wave_generator(&config, frequency as f32);
    let mut triangle_gen = triangle_wave_generator(&config, frequency as f32);

    // Write each waveform to a WAV file
    write_waveform_to_wav(&mut sine_gen, num_samples, &test_audio_name("optimized_sine"));
    write_waveform_to_wav(&mut square_gen, num_samples, &test_audio_name("optimized_square"));
    write_waveform_to_wav(&mut sawtooth_gen, num_samples, &test_audio_name("optimized_sawtooth"));
    write_waveform_to_wav(&mut triangle_gen, num_samples, &test_audio_name("optimized_triangle"));
}

fn write_waveform_to_wav(generator: &mut WaveformGenerator, num_samples: usize, file_name: &str) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = hound::WavWriter::create(file_name, spec).unwrap();

    for _ in 0..num_samples {
        let sample = generator.next_sample();
        writer.write_sample(sample as f32).unwrap();
    }
    writer.finalize().unwrap();
    println!("Completed writing waveform to {}", file_name);
}


fn write_sequence_to_file(config: &SynthConfig, sequence: &[f32], label: &str) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: config.sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Int,
    };
    let filename = test_audio_name(label);
    let mut writer = hound::WavWriter::create(filename.clone(), spec).unwrap();

    for &sample in sequence {
        writer.write_sample(sample).unwrap(); 
    }
    writer.finalize().unwrap();
    println!("Completed writing test waveform {}", filename);
}

fn main() {
    let config = SynthConfig::new(96000, 20.0, 20000.0, 1.0, 0.0, 0.0, 1.0);
    
    // test_write_waveforms(&config);
    test_waveform_generator(&config);
    // test_write_sequenced_melody(&config);
}
