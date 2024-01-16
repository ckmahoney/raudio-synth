use crate::synth_config::SynthConfig;
use std::f32::consts::PI;
use rand::prelude::*;

pub fn sine(config: &SynthConfig, t: u32, freq: f32, _bias: Option<f32>) -> f32 {
    let adjusted_freq = freq + config.tuning_offset_hz;
    let phase = t as f32 * adjusted_freq * 2.0 * PI / config.sample_rate as f32;
    (phase + config.phase_offset).sin() * config.amplitude_scaling
}

pub fn sawtooth(config: &SynthConfig, t: u32, freq: f32, bias: Option<f32>) -> f32 {
    let adjusted_freq = freq + config.tuning_offset_hz;
    let pos = (t as f32 * adjusted_freq % config.sample_rate as f32) / config.sample_rate as f32;
    let bias_val = bias.unwrap_or(0.5);
    2.0 * (pos - bias_val) * config.amplitude_scaling
}

pub fn pulsewidth(config: &SynthConfig, t: u32, freq: f32, width: f32, bias: Option<f32>) -> f32 {
    let adjusted_freq = freq + config.tuning_offset_hz;
    let phase = t as f32 * adjusted_freq / config.sample_rate as f32;
    let bias_val = bias.unwrap_or(0.5);
    if phase % 1.0 < width {
        (1.0 - bias_val) * config.amplitude_scaling
    } else {
        (-1.0 + bias_val) * config.amplitude_scaling
    }
}


pub fn triangle(config: &SynthConfig, t: u32, freq: f32, _bias: Option<f32>) -> f32 {
    let adjusted_freq = freq + config.tuning_offset_hz;
    let phase = t as f32 * adjusted_freq / config.sample_rate as f32;
    2.0 * phase.abs().rem_euclid(2.0) - 1.0
}

pub fn white_noise(config: &SynthConfig) -> f32 {
    let mut rng = thread_rng();
    rng.gen_range(-1.0..1.0) * config.amplitude_scaling
}

pub fn pink_noise(config: &SynthConfig) -> f32 {
    // Pink noise generation can be more complex. This is a simple approximation.
    let mut rng = thread_rng();
    let samples: Vec<f32> = (0..5).map(|_| rng.gen_range(-1.0..1.0)).collect();
    samples.iter().sum::<f32>() / samples.len() as f32 * config.amplitude_scaling
}

// Additional popular waveforms can be square and FM synthesis
pub fn square(config: &SynthConfig, t: u32, freq: f32, _bias: Option<f32>) -> f32 {
    let adjusted_freq = freq + config.tuning_offset_hz;
    let phase = t as f32 * adjusted_freq / config.sample_rate as f32;
    (if phase % 1.0 < 0.5 { 1.0 } else { -1.0 }) * config.amplitude_scaling
}

pub fn fm_synth(config: &SynthConfig, t: u32, carrier_freq: f32, mod_freq: f32, mod_index: f32) -> f32 {
    let adjusted_carrier_freq = carrier_freq + config.tuning_offset_hz;
    let adjusted_mod_freq = mod_freq + config.tuning_offset_hz;
    let carrier_phase = t as f32 * adjusted_carrier_freq / config.sample_rate as f32;
    let modulator_phase = t as f32 * adjusted_mod_freq / config.sample_rate as f32;
    let fm_phase = carrier_phase + mod_index * modulator_phase.sin();
    fm_phase.sin() * config.amplitude_scaling
}


pub fn of(config: &SynthConfig, ts: Vec<u32>, sr:u32, shape: i8) -> Vec<f32> {
    let func = match shape {
        0 => triangle,
        1 => sawtooth,
        2 => square,
        _ => sine
    };

    let mut samples: Vec<f32> = Vec::new();
    let freq: f32 = 400.0;
    let amp = 0.1;
    for t in ts {
        let sample = amp * func(config, t, freq, Some(0.5));
        samples.push(sample);
    }
    let xx = samples.iter().take(10).map(|&x| x.to_string())
    .collect::<Vec<_>>()
    .join("\n");
    println!("For shape ${:?} \nit has these samples", shape);
    println!("{:?}",xx);
    samples
}


#[cfg(test)]
mod tests {
    use super::*;

    #[macro_export]
    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr, $epsilon:expr) => {
            assert!(($a as f32).abs() - ($b as f32).abs() < $epsilon, "assertion failed: `(left ≈ right)`\n  left: `{:?}`,\n right: `{:?}`", $a, $b);
        };
    }


    #[test]
    fn test_sine() {
        let config = SynthConfig::new(96000, 20.0, 20000.0, 1.0, 0.0, 0.0);
        let epsilon = 1e-4;

        // Test at various points in the sine wave cycle
        assert_approx_eq!(0.0, sine(&config, 0, 1.0, None), epsilon);
        assert_approx_eq!(1.0, sine(&config, 24000, 1.0, None), epsilon);
        assert_approx_eq!(0.0, sine(&config, 48000, 1.0, None), epsilon);
        assert_approx_eq!(-1.0, sine(&config, 72000, 1.0, None), epsilon);
        assert_approx_eq!(0.0, sine(&config, 96000, 1.0, None), epsilon);
    }

    #[test]
    fn test_sawtooth() {
        let config = SynthConfig::new(96000, 20.0, 20000.0, 1.0, 0.0, 0.0);
        let epsilon = 1e-4;
    
        assert_eq!(-1.0, sawtooth(&config, 0, 1.0, None));
        assert_eq!(0.0, sawtooth(&config, 48000, 1.0, None));
        assert_approx_eq!(1.0, sawtooth(&config, 95999, 1.0, None), epsilon);
        assert_eq!(-1.0, sawtooth(&config, 96000, 1.0, None));
    
        assert_eq!(-1.0, sawtooth(&config, 0, 2.0, None));
        assert_eq!(0.0, sawtooth(&config, 24000, 2.0, None));
    }
}