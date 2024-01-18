
# raudio-synth

## Example

The following example demonstrates how to generate a sine wave and output it to an audio buffer:

```rust
use raudio-synth::gen::sine_wave_generator;
use raudio-synth::freq_forms::render;
use raudio-synth::synth_config::SynthConfig;
use hound;

let config = SynthConfig {
    sample_rate: 44100,
    min_frequency: 20.0,
    max_frequency: 20000.0,
    volume: 1.0,
    pan: 0.0,
    reverb: 0.0,
    delay: 1.0,
};
let mut generator = sine_wave_generator(&config, 440.0);
let samples = render(&config, (0..44100).collect(), 44100, &generator.next_sample);

// Writing the sine wave audio buffer to a WAV file using hound
let spec = hound::WavSpec {
    channels: 1,
    sample_rate: config.sample_rate,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
};

let mut writer = hound::WavWriter::create("sine_wave.wav", spec).unwrap();
for sample in samples.iter() {
    writer.write_sample(*sample as i16).unwrap();
}
writer.finalize().unwrap();
```

## Tests

To run the unit tests for raudio-synth, execute:
