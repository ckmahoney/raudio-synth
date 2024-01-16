use clap::{App, Arg};
use hound;

mod synth_config;
mod waveforms;
mod wavelets;

use synth_config::SynthConfig;
use waveforms::Ugen;
use std::collections::HashMap;

const TEST_AUDIO_DIR: &str = "test-render";

fn test_audio_name(label:&str) -> String {
    format!("{}/{}.wav", TEST_AUDIO_DIR, label)
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

    let sequence = waveforms::render(config, ts, config.sample_rate, ugen);
    for sample in sequence {
        writer.write_sample(sample).unwrap();
    }
    writer.finalize().unwrap();
    filename
}


fn test_write_waveforms(config: &SynthConfig) {
    let mut shapes_map: HashMap<String, Ugen> = HashMap::new();
    shapes_map.insert(String::from("sawtooth"), waveforms::sawtooth);
    shapes_map.insert(String::from("triangle"), waveforms::triangle);
    shapes_map.insert(String::from("sine"), waveforms::sine);

    for (name, func) in &shapes_map {
        let filename = render_ugen(&config, func, name);
        println!("Completed writing test waveform {}", filename);
    }
}

fn main() {
    let _melody = [
        400, 600, 500, 700, 800, 600, 500, 400 
    ];
    let config = SynthConfig::new(96000, 20.0, 20000.0, 1.0, 0.0, 0.0);
    
    test_write_waveforms(&config);
}
