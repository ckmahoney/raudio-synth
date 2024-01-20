//! Methods for creating slices of useful waveforms. 
//! Intended to be applied as a modulator to phase, amplitude, or frequency.

struct Envelope {
    n: usize,
    sample_rate: i32,
    flip: bool,
    cps: f32
}

impl Envelope {
    fn new(n: usize, sample_rate: i32, cps: f32, flip: bool) -> Envelope {
        if cps <= 0.0 {
            panic!("CPS must be a tempo")
        }
        Envelope { n, sample_rate, cps, flip }
    }

    fn normalize_and_flip(&self, samples: &mut Vec<f32>) {
        let max_val: f32 = samples.iter().fold(0.0, |max, &val| max.max(val.abs()));
        let sign = if self.flip { -1.0 } else { 1.0 };
    
        if max_val > 1.0 {
            for sample in samples.iter_mut() {
                *sample = sign * (*sample / max_val);
            }
        } else {
            if self.flip {
                for sample in samples.iter_mut() {
                    *sample *= sign;
                }
            }
        }
    }

    pub fn constant(&self, x: f32) -> Vec<f32> {
        if x > 1.0 || x < -1.0 {
            panic!("Modulation samples must be bound to [-1.0, 1.0].");
        }

        vec![x; self.n]
    }

    pub fn linear(&self, slope: f32) -> Vec<f32> {
        let mut samples = Vec::with_capacity(self.n);
        for i in 0..self.n {
            let value = slope * (i as f32 / self.sample_rate as f32);
            samples.push(value);
        }
        self.normalize_and_flip(&mut samples);
        samples
    }

    pub fn power(&self, base: f32, pow: f32) -> Vec<f32> {
        let mut samples = Vec::with_capacity(self.n);
        for i in 0..self.n {
            let t = i as f32 / self.sample_rate as f32;
            let value = (t / base).powf(pow);
            samples.push(value);
        }
        self.normalize_and_flip(&mut samples);
        samples
    }

    pub fn exponential(&self, base: f32, pow: f32) -> Vec<f32> {
        let mut samples = Vec::with_capacity(self.n);
        for i in 0..self.n {
            let t = i as f32 / self.sample_rate as f32;
            let value = base.powf(t * pow);
            samples.push(value);
        }
        self.normalize_and_flip(&mut samples);
        samples
    }
}

#[cfg(test)]
mod tests {
    use rand::{distributions::Uniform, Rng};

    use super::*;
    

    #[test]
    fn test_constant() {
        let envelope = Envelope::new(5, 44100, 1.2, false);
        let result = envelope.constant(0.5);
        assert_eq!(result, vec![0.5, 0.5, 0.5, 0.5, 0.5]);
    }

    #[test]
    fn test_linear() {
        let envelope = Envelope::new(5, 1, 1.2, false);
        let result = envelope.linear(1.0);
        let expected = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        assert_eq!(result, expected);
    }
    #[test]
    fn test_power_increasing() {
        let n_runs = 10;
        let mut rng = rand::thread_rng();
        let sample_rate_range = Uniform::new(1, 44100);
        let base_range = Uniform::new(1.0, 5.0);
        let pow_range = Uniform::new(0.0, 5.0);

        for _ in 0..n_runs {
            let sample_rate = rng.sample(sample_rate_range);
            let base = rng.sample(base_range);
            let pow = rng.sample(pow_range);
            let envelope = Envelope::new(10, sample_rate, 1.2, false);
            let result = envelope.power(base, pow);

            for i in 0..(result.len() - 1) {
                assert!(result[i] <= result[i + 1], "Values are not increasing: {:?}", result);
            }
        }
    }

    #[test]
    fn test_power_decreasing() {
        let n_runs = 10;
        let mut rng = rand::thread_rng();
        let sample_rate_range = Uniform::new(1, 44100);
        let base_range = Uniform::new(1.0, 5.0);
        let pow_range = Uniform::new(0.0, 5.0);

        for _ in 0..n_runs {
            let sample_rate = rng.sample(sample_rate_range);
            let base = rng.sample(base_range);
            let pow = rng.sample(pow_range);
            let envelope = Envelope::new(10, sample_rate, 1.2, true);
            let result = envelope.power(base, pow);

            for i in 0..(result.len() - 1) {
                assert!(result[i] >= result[i + 1], "Values are not decreasing: {:?}", result);
            }
        }
    }

    #[test]
    fn test_exponential_increasing() {
        let envelope = Envelope::new(10, 1, 1.2, false);
        let result = envelope.exponential(2.0, 1.0);
        for i in 0..(result.len() - 1) {
            assert!(result[i] <= result[i + 1]);
        }
    }
    
    #[test]
    fn test_exponential_decreasing() {
        let envelope = Envelope::new(10, 1, 1.2, true);
        let result = envelope.exponential(2.0, 1.0);
        for i in 0..(result.len() - 1) {
            assert!(result[i] >= result[i + 1]);
        }
    }

    #[test]
    fn test_flip() {
        let envelope = Envelope::new(5, 1, 1.2, true);
        let result = envelope.linear(1.0);
        let expected = vec![0.0, -0.25, -0.5, -0.75, -1.0];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_normalize_multiple_runs() {
        let n_runs = 10;
        let mut rng = rand::thread_rng();
        let sample_rate_range = Uniform::new(1, 44100);
        let cps_range = Uniform::new(0.1, 5.0);
        let base_range = Uniform::new(1.0, 5.0);
        let pow_range = Uniform::new(0.0, 5.0);

        for _ in 0..n_runs {
            let sample_rate = rng.sample(sample_rate_range);
            let cps = rng.sample(cps_range);
            let base = rng.sample(base_range);
            let pow = rng.sample(pow_range);
            let envelope = Envelope::new(4, sample_rate, cps, rng.gen_bool(0.5));
            let result = envelope.exponential(base, pow);

            assert!(result.iter().all(|&x| x >= -1.0 && x <= 1.0),
                "Result not normalized: {:?}", result);
        }
    }

    #[test]
    #[should_panic(expected = "Modulation samples must be bound to [-1.0, 1.0].")]
    fn test_constant_out_of_range() {
        let envelope = Envelope::new(5, 44100, 1.2, false);
        envelope.constant(1.5);
    }
}