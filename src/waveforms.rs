use std::f32::consts::PI;

pub fn sine(t: u32, freq: f32, _bias: f32, sr: u32) -> f32 {
    let phase = t as f32 * freq * 2.0 * PI / sr as f32;
    phase.sin()
}

pub fn sawtooth(t: u32, freq: f32, _bias: f32, sr: u32) -> f32 {
    let pos = (t as f32 * freq % sr as f32) /sr as f32;
    2.0 * (pos - 0.5)
}

#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr, $epsilon:expr) => {
        assert!(($a as f32).abs() - ($b as f32).abs() < $epsilon, "assertion failed: `(left ≈ right)`\n  left: `{:?}`,\n right: `{:?}`", $a, $b);
    };
}

pub fn triangle(t: u32, freq: f32, _bias: f32, sr: u32) -> f32 {
    let pos = (t as f32 * freq / sr as f32).fract();
    2.0 * (1.0 - (2.0 * pos - 1.0).abs()) - 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sine() {
        let epsilon = 1e-4;

        // Test at various points in the sine wave cycle
        assert_approx_eq!(0.0, sine(0, 1.0, 0.5, 96000), epsilon);
        assert_approx_eq!(1.0, sine(24000, 1.0, 0.5, 96000), epsilon);
        assert_approx_eq!(0.0, sine(48000, 1.0, 0.5, 96000), epsilon);
        assert_approx_eq!(-1.0, sine(72000, 1.0, 0.5, 96000), epsilon);
        assert_approx_eq!(0.0, sine(96000, 1.0, 0.5, 96000), epsilon);
    }

    #[test]
    fn test_sawtooth() {
        let epsilon = 1e-4;
    
        assert_eq!(-1.0, sawtooth(0, 1.0, 0.5, 96000));
        assert_eq!(0.0, sawtooth(48000, 1.0, 0.5, 96000));
        assert_approx_eq!(1.0, sawtooth(95999, 1.0, 0.5, 96000), epsilon);
        assert_eq!(-1.0, sawtooth(96000, 1.0, 0.5, 96000));
    
        assert_eq!(-1.0, sawtooth(0, 2.0, 0.5, 96000));
        assert_eq!(0.0, sawtooth(24000, 2.0, 0.5, 96000));
        assert_approx_eq!(1.0, sawtooth(47999, 2.0, 0.5, 96000), epsilon);
        assert_eq!(-1.0, sawtooth(48000, 2.0, 0.5, 96000));
        assert_eq!(-1.0, sawtooth(96000, 2.0, 0.5, 96000));
    }
    
    #[test]
    fn test_triangle() {
        let epsilon = 1e-4;

        assert_eq!(-1.0, triangle(0, 1.0, 0.5, 96000));
        assert_approx_eq!(0.0, triangle(24000, 1.0, 0.5, 96000), epsilon);
        assert_approx_eq!(1.0, triangle(48000, 1.0, 0.5, 96000), epsilon);
        assert_approx_eq!(0.0, triangle(72000, 1.0, 0.5, 96000), epsilon);
        assert_eq!(-1.0, triangle(96000, 1.0, 0.5, 96000));

        assert_eq!(-1.0, triangle(0, 2.0, 0.5, 96000));
        assert_approx_eq!(0.0, triangle(12000, 2.0, 0.5, 96000), epsilon);
        assert_approx_eq!(1.0, triangle(24000, 2.0, 0.5, 96000), epsilon);
        assert_approx_eq!(0.0, triangle(36000, 2.0, 0.5, 96000), epsilon);
        assert_eq!(-1.0, triangle(48000, 2.0, 0.5, 96000));
        assert_eq!(-1.0, triangle(96000, 2.0, 0.5, 96000));
    }

    #[test]
    fn test_square() {

        assert_eq!(-1.0, square(0, 1.0, 0.5, 96000));
        assert_eq!(-1.0, square(24000, 1.0, 0.5, 96000));
        assert_eq!(-1.0, square(47999, 1.0, 0.5, 96000));
        assert_eq!(1.0, square(48000, 1.0, 0.5, 96000));
        assert_eq!(1.0, square(72000, 1.0, 0.5, 96000));
        assert_eq!(1.0, square(95999, 1.0, 0.5, 96000));
        assert_eq!(-1.0, square(96000, 1.0, 0.5, 96000));

        assert_eq!(-1.0, square(0, 2.0, 0.5, 96000));
        assert_eq!(-1.0, square(12000, 2.0, 0.5, 96000));
        assert_eq!(-1.0, square(23999, 2.0, 0.5, 96000));
        assert_eq!(1.0, square(36000, 2.0, 0.5, 96000));
        assert_eq!(1.0, square(42000, 2.0, 0.5, 96000));
        assert_eq!(-1.0, square(48000, 2.0, 0.5, 96000));
        assert_eq!(-1.0, square(96000, 2.0, 0.5, 96000));
    }
}

pub fn square(t: u32, freq: f32, _bias: f32, sr: u32) -> f32 {
    let pos = (t as f32 * freq) % sr as f32; 
    if (pos/ sr as f32).fract() < 0.5 { -1.0 } else { 1.0 }
}


pub fn of(ts: Vec<u32>, sr:u32, shape: i8) -> Vec<f32> {
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
        let sample = amp * func(t, freq, 0.5, sr);
        samples.push(sample);
    }
    let xx = samples.iter().take(10).map(|&x| x.to_string())
    .collect::<Vec<_>>()
    .join("\n");
    println!("For shape ${:?} \nit has these samples", shape);
    println!("{:?}",xx);
    samples
}
