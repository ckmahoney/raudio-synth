
//! Methods for creating slices of useful waveforms. 
//! Intended to be applied as a modulator to phase, amplitude, or frequency.

struct Envelope {
    n: usize,
    sample_rate: i32,
    amp: f32,
    flip: bool,
}

impl Envelope {
    fn new(n: usize, sample_rate: i32, amp: f32, flip: bool) -> Envelope {
        Envelope { n, sample_rate, amp, flip }
    }

    fn normalize_and_flip(&self, samples: &mut Vec<f32>) {
        let max_val = samples.iter().fold(0.0, |max, &val| max.max(val.abs()));
        let sign = if self.flip { -1.0 } else { 1.0 };
        for sample in samples.iter_mut() {
            *sample = sign * (*sample / max_val);
        }
    }

    pub fn constant(&self, x: f32) -> Vec<f32> {
        vec![x; self.n]
    }

    pub fn linear(&self, slope: f32) -> Vec<f32> {
        let mut samples = Vec::with_capacity(self.n);
        for i in 0..self.n {
            let value = self.amp + slope * (i as f32 / self.sample_rate as f32);
            samples.push(value);
        }
        self.normalize_and_flip(&mut samples);
        samples
    }

    pub fn power(&self, base: f32, pow: f32) -> Vec<f32> {
        let mut samples = Vec::with_capacity(self.n);
        for i in 0..self.n {
            let t = i as f32 / self.sample_rate as f32;
            let value = (self.amp + (t / base).powf(pow));
            samples.push(value);
        }
        self.normalize_and_flip(&mut samples);
        samples
    }

    pub fn exponential(&self, base: f32, pow: f32) -> Vec<f32> {
        let mut samples = Vec::with_capacity(self.n);
        for i in 0..self.n {
            let t = i as f32 / self.sample_rate as f32;
            let value = self.amp + base.powf(t * pow);
            samples.push(value);
        }
        self.normalize_and_flip(&mut samples);
        samples
    }
}