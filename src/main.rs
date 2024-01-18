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
    let dur_cycles = 4;
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: config.sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Int,
    };
    let name: String = format!("{}_sample-rate_{}_channels_{}", label, spec.sample_rate, spec.channels);
    let filename = setup_tests::test_audio_name(&name);
    let mut writer = hound::WavWriter::create(filename.clone(), spec).unwrap();
    let mut ts: Vec<u32> = Vec::new();

    for i in 0..(dur_cycles * config.sample_rate) { 
        ts.push(i) 
    };

    let sequence = render(config, ts, config.sample_rate, ugen, 440.0, 1.0);
    for sample in sequence {
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
        freq_forms::sawtooth,
        freq_forms::triangle,
        freq_forms::sine,
    ];

    let mut rng = thread_rng();
    let mut complete_sequence: Vec<f32> = Vec::new();

    for (index, &frequency) in melody.iter().enumerate() {
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
    let frequency = 400;
    let duration_in_seconds = 2;
    let num_samples = (sample_rate * duration_in_seconds as f64) as usize;

    // Generate waveforms
    let mut sine_gen = sine_wave_generator(&config, frequency as f32);
    let mut square_gen = square_wave_generator(&config, frequency as f32);
    let mut sawtooth_gen = sawtooth_wave_generator(&config, frequency as f32);
    let mut triangle_gen = triangle_wave_generator(&config, frequency as f32);

    // Write each waveform to a WAV file
    write_waveform_to_wav(&mut sine_gen, num_samples, &setup_tests::test_audio_name("optimized_sine"));
    write_waveform_to_wav(&mut square_gen, num_samples, &setup_tests::test_audio_name("optimized_square"));
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
    println!("Hello, world!");
}
    writer.finalize().unwrap();
    println!("Completed writing test waveform {}", filename);
}

fn main() {
    let config = setup_tests::generate_synth_config();
    
    // test_write_waveforms(&config);
    test_waveform_generator(&config);
    // test_write_sequenced_melody(&config);
}
