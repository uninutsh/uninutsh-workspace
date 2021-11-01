use std::f64::consts;

pub fn square(i: f64) -> [f64; 2] {
    if i <= 0.5 {
        return [1.0, 1.0];
    }
    return [-1.0, -1.0];
}
pub fn saw(i: f64) -> [f64; 2] {
    let s = 1.0 - i * 2.0;
    return [s, s];
}
pub fn sine(i: f64) -> [f64; 2] {
    let s = (i * 2.0 * consts::PI).sin();
    return [s, s];
}

pub fn algebraic_soft_old(i: f64) -> [f64; 2] {
    if i < 0.5 {
        let s = -(2.0) * (4.0 * (i - 0.25)) / (1.0 + 4.0 * (i - 0.25).abs());
        return [s, s];
    }
    let s = (2.0) * (4.0 * (i - 0.75)) / (1.0 + 4.0 * (i - 0.75).abs());
    return [s, s];
}

pub fn algebraic_soft(i: f64) -> [f64; 2] {
    if i < 0.5 {
        let s = -(2.0) * (4.0 * (i - 0.25)) / (1.0 + 4.0 * (i - 0.25).abs());
        return [s, s];
    }
    let s = (2.0) * (4.0 * (i - 0.75)) / (1.0 + 4.0 * (i - 0.75).abs());
    return [s, s];
}

pub fn algebraic(i: f64) -> [f64; 2] {
    let s = -(3.0 / 2.0) * (4.0 * (i - 0.5)) / (1.0 + 4.0 * (i - 0.5).abs());
    return [s, s];
}

pub fn sigmoid(i: f64) -> [f64; 2] {
    let s = -2f64.sqrt() * (2.0 * i - 1.0) / (1.0 + (2.0 * i - 1.0).powi(2)).sqrt();
    return [s, s];
}

pub fn sigmoid_soft(i: f64) -> [f64; 2] {
    if i < 0.5 {
        let s = -5f64.sqrt() * (2.0 * i - 0.5) / (1.0 + (2.0 * i - 0.5).powi(2)).sqrt();
        return [s, s];
    }
    let s = -5f64.sqrt() * (2.0 * i - 1.5) / (1.0 + (2.0 * i - 1.5).powi(2)).sqrt();
    return [s, s];
}
