#[cfg(test)]
mod tests {
    use raudio_synth::synth_config::SynthConfig;
    use raudio_synth::freq_forms::{sine, square, sawtooth, triangle};

    #[test]
    fn test_sine_wave_properties() {
        let config = SynthConfig::new(44100, 20.0, 20000.0, 1.0, 0.0, 0.0, 1.0);
        let freq = 440.0;
        let samples: Vec<f32> = (0..44100).map(|t| sine(&config, t, freq, None)).collect();

        assert!(samples.iter().all(|&sample| sample >= -1.0 && sample <= 1.0));
        assert_eq!(samples[0], 0.0);
        assert!(samples[11025].abs() > 0.999);
        assert_eq!(samples[22050], 0.0);
    }

    #[test]
    fn test_square_wave_properties() {
        let config = SynthConfig::new(44100, 20.0, 20000.0, 1.0, 0.0, 0.0, 1.0);
        let freq = 440.0;
        let samples: Vec<f32> = (0..44100).map(|t| square(&config, t, freq, None)).collect();

        assert!(samples.iter().all(|&sample| sample >= -1.0 && sample <= 1.0));
        assert_eq!(samples[0], 1.0);
        assert_eq!(samples[22050], -1.0);
    }

    #[test]
    fn test_sawtooth_wave_properties() {
        let config = SynthConfig::new(44100, 20.0, 20000.0, 1.0, 0.0, 0.0, 1.0);
        let freq = 440.0;
        let samples: Vec<f32> = (0..44100).map(|t| sawtooth(&config, t, freq, None)).collect();

        assert!(samples.iter().all(|&sample| sample >= -1.0 && sample <= 1.0));
        assert_eq!(samples[0], -1.0);
        assert_eq!(samples[22050], 1.0);
    }

    #[test]
    fn test_triangle_wave_properties() {
        let config = SynthConfig::new(44100, 20.0, 20000.0, 1.0, 0.0, 0.0, 1.0);
        let freq = 440.0;
        let samples: Vec<f32> = (0..44100).map(|t| triangle(&config, t, freq, None)).collect();

        assert!(samples.iter().all(|&sample| sample >= -1.0 && sample <= 1.0));
        assert_eq!(samples[0], 0.0);
        assert_eq!(samples[11025], -1.0);
        assert_eq!(samples[22050], 0.0);
    }
}
