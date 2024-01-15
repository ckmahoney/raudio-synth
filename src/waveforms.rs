pub fn sawtooth(t: f32, freq: f32) -> f32 {
    2.0 * (t * freq - (0.5 + t * freq).floor())
}

pub fn triangle(t: f32, freq: f32) -> f32 {
    2.0 * sawtooth(t, freq).abs() - 1.0
}

pub fn square(t: f32, freq: f32) -> f32 {
    if sawtooth(t, freq) >= 0.0 { 1.0 } else { -1.0 }
}
