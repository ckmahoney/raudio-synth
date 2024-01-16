use clap::{App, Arg};
use hound;

mod waveforms;
mod composer;
mod wavelets;

fn h(shape: u32) {
    let sample_rate = 96000;
    let filename = "anew_sequence.wav";
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(filename, spec).unwrap();
    // let sequence1 = composer::compose_sequence(10.0, 44100); 
    let mut ts: Vec<u32> = Vec::new();
    for i in 0..sample_rate { 
        ts.push(i) 
    };

    let sequence = waveforms::of(ts, sample_rate, shape);
    for sample in sequence {
        writer.write_sample(sample).unwrap();
    }

    wavelets::main(440.0)
}

fn main() {
    let arg_matches = App::new("My App")
        .arg(Arg::with_name("waveshape")
            .short('w')
            .long("waveshape")
            .takes_value(true)
            .help("Select the waveshape to render. Choose from 0, 1, or 2."))
        .get_matches();

    match arg_matches.value_of("waveshape").unwrap_or("0").parse::<u32>() {
        Ok(shape) if shape <= 2 => {
            println!("waveshape: {}", shape);
            h(shape)
        },
        _ => eprintln!("Invalid waveshape. Please enter 0, 1, or 2."),
    }
}
