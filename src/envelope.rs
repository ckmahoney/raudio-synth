
//! Methods for creating slices of useful waveforms. 
//! Intended to be applied as a modulator to phase, amplitude, or frequency. 

pub fn constant(n: usize, x: f32) -> Vec<f32> {
    if x < -1.0 || x > 1.0 {
        panic!("Modulation samples must be bound by an audio signal's range.")
    }
    vec![x; n]
}

pub fn linear(n: usize, sample_rate: i32, amp: f32, flip: bool, slope: f32) -> Vec<f32> {
    if slope == 0.0 {
        panic!("Cannot have a slope of 0 for a linear envelope. Use amp = 0 instead.");
    }
    let mut samples: Vec<f32> = Vec::with_capacity(n);
    let sign = if flip { -1.0 } else { 1.0 };
    let mut max_val: f32 = 0.0;
    for i in 0..n {
        let value = amp + sign * slope * (i as f32 / sample_rate as f32);
        max_val = max_val.max(value.abs());
        samples.push(value);
    }
    samples.iter().map(|&x| x / max_val).collect()
}

pub fn power(n: usize, sample_rate: i32, amp: f32, base: f32, pow: f32, flip: bool) -> Vec<f32> {
    if base == 0.0 || pow == 0.0 {
        panic!("Cannot have a base or power of 0 for a power envelope. Use amp = 0 or amp = 1 instead.");
    }
    let mut samples: Vec<f32> = Vec::with_capacity(n);
    let sign = if flip { -1.0 } else { 1.0 };
    let mut max_val: f32 = 0.0;
    for i in 0..n {
        let t = i as f32 / sample_rate as f32;
        let value = sign * (amp + (t / base).powf(pow));
        max_val = max_val.max(value.abs());
        samples.push(value);
    }
    samples.iter().map(|&x| x / max_val).collect()
}

pub fn exponential(n: usize, sample_rate: i32, amp: f32, base: f32, pow: f32, flip: bool) -> Vec<f32> {
    if base == 0.0 || pow == 0.0 {
        panic!("Cannot have a base or power of 0 for an exponential envelope. Use amp = 0 or amp = 1 instead.");
    }
    let mut samples: Vec<f32> = Vec::with_capacity(n);
    let sign = if flip { -1.0 } else { 1.0 };
    let mut max_val:f32 = 0.0;
    for i in 0..n {
        let t = i as f32 / sample_rate as f32;
        let value = sign * (amp + base.powf(t * pow));
        max_val = max_val.max(value.abs());
        samples.push(value);
    }
    samples.iter().map(|&x| x / max_val).collect()
}