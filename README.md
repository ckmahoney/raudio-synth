
# raudio-synth

## Example <- COMMENT: great example, thanks

The following example demonstrates how to generate a sine wave and output it to an audio buffer:

```rust
use raudio-synth::gen::sine_wave_generator;
use raudio-synth::freq_forms::render;
use raudio-synth::synth_config::SynthConfig;

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
```

## Tests

To run the unit tests for raudio-synth, execute:

```shell
cargo test
```
