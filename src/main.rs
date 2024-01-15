use hound;
use std::i16;

mod waveforms;
mod composer;
mod wavelets;

fn main() {
    let filename = "random_sequence.wav";
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(filename, spec).unwrap();
    let sequence = composer::compose_sequence(10.0, 44100); // 10 seconds sequence
    for sample in sequence {
        writer.write_sample((sample * i16::MAX as f32) as i16).unwrap();
    }

    wavelets::main(440.0)
}
