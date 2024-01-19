use crate::synth_config::SynthConfig;

pub type Ugen = fn(&SynthConfig, u32, f32, Option<f32>) -> f32;

pub fn render_ugen(config: &SynthConfig, ugen: &Ugen, filename: &str) -> String {
    let dur_cycles = 4;
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: config.sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(filename, spec).unwrap();
    let mut ts: Vec<u32> = Vec::new();

    for i in 0..(dur_cycles * config.sample_rate) { 
        ts.push(i) 
    };

    let sequence = render2(config, ts, config.sample_rate, ugen, 440.0, 1.0);
    for sample in sequence {
        writer.write_sample(sample).unwrap();
    }
    writer.finalize().unwrap();
    String::from("done")
}

pub fn render2(config: &SynthConfig, ts: Vec<u32>, sr:u32, ugen: &Ugen, freq: f32, amp: f32) -> Vec<f32> {
    let mut samples: Vec<f32> = Vec::new();
    for t in ts {
        let sample = amp * ugen(config, t, freq, Some(0.5));
        samples.push(sample);
    }
    samples
}

