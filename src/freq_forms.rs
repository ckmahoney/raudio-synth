use std::f32::consts::PI;
use crate::synth_config::SynthConfig;

pub fn normalize_waveform(samples: &mut [f32]) {
    let (min, max) = samples.iter().fold((f32::MAX, f32::MIN), |(min, max), &val| {
        (min.min(val), max.max(val))
    });
    let amplitude_range = max - min;
    samples.iter_mut().for_each(|sample| {
        *sample = (*sample - min) / amplitude_range * 2.0 - 1.0;
    });
}

pub fn sine(sample_num: u32, sample_rate: u32, frequency: f32, phase_offset: f32) -> f32 {
    let t = sample_num as f32 / sample_rate as f32;
    (2.0 * PI * frequency * t + phase_offset).sin()
} 

pub fn square(sample_num: u32, sample_rate: u32, frequency: f32, phase_offset: f32) -> f32 {
    let nyquist = sample_rate as f32 / 2.0;
    let max_harmonic = (nyquist / frequency).floor() as i32;
    let t = sample_num as f32 / sample_rate as f32;
    (1..=max_harmonic).step_by(2).fold(0.0, |acc, n| {
        acc + ((2.0 * PI * frequency * n as f32 * t + phase_offset).sin() / n as f32)
    })
}

pub fn sawtooth(config: &SynthConfig, t: u32, freq: f32, bias: Option<f32>) -> f32 {
    let adjusted_freq = freq + config.tuning_offset_hz;
    let nyquist = config.sample_rate as f32 / 2.0;
    let max_harmonic = (nyquist / adjusted_freq).floor() as i32;
    let t = t as f32 / config.sample_rate as f32;
    let mut sum = 0.0;
    for n in 1..=max_harmonic {
        let harmonic_bias = (n as f32 * bias.unwrap_or(0.5)).rem_euclid(1.0);
        sum += (2.0 * std::f32::consts::PI * adjusted_freq * n as f32 * t + config.phase_offset + harmonic_bias).sin() / n as f32;
    }
    sum * config.amplitude_scaling
}

pub fn triangle(config: &SynthConfig, t: u32, freq: f32, bias: Option<f32>) -> f32 {
    let adjusted_freq = freq + config.tuning_offset_hz;
    let nyquist = config.sample_rate as f32 / 2.0;
    let max_harmonic = (nyquist / adjusted_freq).floor() as i32;
    let t = t as f32 / config.sample_rate as f32;
    let mut sum = 0.0;
    for n in (1..=max_harmonic).step_by(2) {
        let harmonic_bias = (n as f32 * bias.unwrap_or(0.5)).rem_euclid(1.0);
        sum += (2.0 * std::f32::consts::PI * adjusted_freq * n as f32 * t + config.phase_offset + harmonic_bias).sin() / (n as f32).powi(2);
    }
    sum * config.amplitude_scaling
}

pub fn render(config: &SynthConfig, ts: Vec<u32>, sr: u32, ugen: fn(&SynthConfig, u32, f32, Option<f32>) -> f32) -> Vec<f32> {
    let mut samples: Vec<f32> = Vec::with_capacity(ts.len());
    let freq: f32 = 400.0;
    let amp = 1.0;

    for t in ts {
        let sample = amp * ugen(config, t, freq, Some(0.5));
        samples.push(sample);
    }

    normalize_waveform(&mut samples);

    samples
}

#[cfg(test)]
mod tests {
    use super::*;

    // Define a basic SynthConfig for testing
    fn test_config() -> SynthConfig {
        SynthConfig {
            sample_rate: 44100,
            min_frequency: 20.0,
            max_frequency: 20000.0,
            amplitude_scaling: 1.0,
            phase_offset: 0.0,
            tuning_offset_hz: 0.0,
        }
    }

    #[test]
    fn test_square_wave_basic() {
        let config = test_config();
        let sample = square(0, config.sample_rate, 440.0, 0.0);
        assert!(sample >= -1.0 && sample <= 1.0, "Square wave sample is not within expected range.");
    }

    #[test]
    fn test_sawtooth_wave_basic() {
        let config = test_config();
        let sample = sawtooth(&config, 0, 440.0, Some(0.5));
        assert!(sample >= -1.0 && sample <= 1.0, "Sawtooth wave sample is not within expected range.");
    }

    #[test]
    fn test_square_wave_frequency_bounds() {
        let config = test_config();
        let low_freq_sample = square(0, config.sample_rate, config.min_frequency, 0.0);
        let high_freq_sample = square(0, config.sample_rate, config.max_frequency, 0.0);
        assert!(low_freq_sample >= -1.0 && low_freq_sample <= 1.0, "Low frequency square wave sample is out of bounds.");
        assert!(high_freq_sample >= -1.0 && high_freq_sample <= 1.0, "High frequency square wave sample is out of bounds.");
    }

    #[test]
    fn test_sawtooth_wave_frequency_bounds() {
        let config = test_config();
        let low_freq_sample = sawtooth(&config, 0, config.min_frequency, Some(0.5));
        let high_freq_sample = sawtooth(&config, 0, config.max_frequency, Some(0.5));
        assert!(low_freq_sample >= -1.0 && low_freq_sample <= 1.0, "Low frequency sawtooth wave sample is out of bounds.");
        assert!(high_freq_sample >= -1.0 && high_freq_sample <= 1.0, "High frequency sawtooth wave sample is out of bounds.");
    }
}