use clap::{App, Arg};
use hound;

mod waveforms;
mod composer;
mod wavelets;

fn h(shape: i8, fp: &str) {
    let sample_rate = 96000;
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Int,
    };
    let filename = fp.to_owned() + ".wav";
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
        .arg(Arg::with_name("filename")
            .short('f')
            .long("filename")
            .takes_value(true)
            .help("Sets the output filename"))
        // .arg(Arg::with_name("format")
        //     .short('o')
        //     .long("format")
        //     .takes_value(true)
        //     .help("Sets the output file format"))
        // .arg(Arg::with_name("transcode")
        //     .short('t')
        //     .long("transcode")
        //     .takes_value(true)
        //     .help("Enable transcoding and specify input file"))
        .get_matches();

    let shape: i8 = arg_matches.value_of("waveshape").unwrap_or("0").parse::<i8>().unwrap_or(3);
    let filename = arg_matches.value_of("filename").unwrap_or("output.wav");
    // let format = arg_matches.value_of("format").unwrap_or("wav");

    println!("Using shape ${:?}", shape);

    h(shape, filename);
}
