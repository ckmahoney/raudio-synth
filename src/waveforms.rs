pub fn sawtooth(t: u32, freq: f32, _bias: f32, sr: u32) -> f32 {
    let pos = (t as f32 * freq % (sr +1) as f32) /sr as f32;
    pos - 0.5
}

#[test]
fn test_sawtooth() {
    assert_eq!(-0.5, sawtooth(0, 1.0, 0.5, 96000));
    assert_eq!(0.0, sawtooth(48000, 1.0, 0.5, 96000));
    assert_eq!(0.5, sawtooth(96000, 1.0, 0.5, 96000));

    assert_eq!(-0.5, sawtooth(0, 2.0, 0.5, 96000));
    assert_eq!(0.0, sawtooth(24000, 2.0, 0.5, 96000));
    assert_eq!(0.5, sawtooth(48000, 2.0, 0.5, 96000));
}

pub fn triangle(t: u32, freq: f32, bias: f32, sr: u32) -> f32 {
    2.0 * sawtooth(t, freq, bias, sr).abs() - 1.0
}

pub fn square(t: u32, freq: f32, bias: f32, sr: u32) -> f32 {
    if sawtooth(t, freq, bias, sr) >= 0.0 { 1.0 } else { -1.0 }
}

pub fn of(ts: Vec<u32>, sr:u32, shape: i8) -> Vec<f32> {
    let func = match shape {
        0 => sawtooth,
        1 => triangle,
        2 => square,
        _ => panic!("Unsupported shape"),
    };

    let mut samples: Vec<f32> = Vec::new();
    let freq: f32 = 400.0;
    for t in ts {
        let sample = func(t, freq, 0.5, sr);
        samples.push(sample);
    }

    samples
}

/*
pub fn sine_wave(t: f32, freq: f32) -> f32 {
    (t * freq * std::f32::consts::PI * 2.0).sin()
}

pub fn pulse_wave(t: f32, freq: f32, duty_cycle: f32) -> f32 {
    if (t * freq) % 1.0 < duty_cycle { 1.0 } else { -1.0 }
}

pub fn white_noise() -> f32 {
    rand::random::<f32>() * 2.0 - 1.0
}

pub fn pink_noise() -> f32 {
    // Pink noise generation involves more complex filtering to achieve a -3dB/octave slope
    // A simple implementation might use a filter on white noise or other methods
    static mut PINK_NOISE_FILTERS: [f32; 7] = [0.0; 7];
    static mut PINK_NOISE_INDEX: usize = 0;

    unsafe {
        PINK_NOISE_INDEX = (PINK_NOISE_INDEX + 1) % 7;
        let white = white_noise();
        
        let mut pink = 0.0;
        for i in 0..7 {
            if PINK_NOISE_INDEX & (1 << i) == (1 << i) {
                PINK_NOISE_FILTERS[i] = 0.99886 * PINK_NOISE_FILTERS[i] + white * 0.0555179;
            }
            pink += PINK_NOISE_FILTERS[i];
        }

        pink * 0.11 // Adjusting the amplitude
    }
}

pub fn brown_noise() -> f32 {
    // Brown noise, or Brownian noise, has a -6dB/octave slope, similar to integrating white noise
    static mut PREV_VALUE: f32 = 0.0;
    unsafe {
        PREV_VALUE = (PREV_VALUE + white_noise() * 0.02).max(-1.0).min(1.0);
        PREV_VALUE
    }
}

pub fn blue_noise() -> f32 {
    // Blue noise, the opposite of pink noise, emphasizes higher frequencies
    static mut PREV_WHITE: f32 = 0.0;
    static mut PREV_BLUE: f32 = 0.0;

    unsafe {
        let white = white_noise();
        PREV_BLUE = 0.995 * PREV_BLUE + (white - PREV_WHITE);
        PREV_WHITE = white;

        PREV_BLUE
    }
}
 */