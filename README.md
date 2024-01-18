
# raudio-synth

To include raudio-synth in your Rust project, add the following to your Cargo.toml file:

```toml
[dependencies]
raudio-synth = { git = "https://github.com/your-username/raudio-synth.git" }
```

## Example <- COMMENT: great example, thanks

The following example demonstrates how to generate a sine wave and output it to an audio buffer:

```rust
use raudio-synth::gen::sine_wave_generator;
use raudio-synth::freq_forms::render;
use raudio-synth::synth_config::SynthConfig;

let sample_rate = 44100.0;
let low_freq = 20.0;
let high_freq = 20000.0;
let amplitude = 1.0;
let pan = 0.0;
let mix = 0.0;
let feedback = 1.0;

let config = SynthConfig::new(sample_rate, low_freq, high_freq, amplitude, pan, mix, feedback);
let mut generator = sine_wave_generator(&config, 440.0);
let samples = render(&config, (0..44100).collect(), 44100, &generator.next_sample);
```

## Tests

To run the unit tests for raudio-synth, execute:
