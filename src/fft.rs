use std::f64::consts::PI;
use num::complex::Complex;

pub const ROUND_TO_DECIMALS: f64 = 10e13;

pub fn round(num: f64) -> f64 {
    let rounded: f64 = (num * ROUND_TO_DECIMALS).round();
    return rounded / ROUND_TO_DECIMALS;
}

pub fn get_even_elements(arr: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let n = arr.len();
    let mut result: Vec<Complex<f64>> = arr.iter().step_by(2).copied().collect();
    result.resize(n / 2, Complex::new(0.0, 0.0));
    result
}

pub fn get_odd_elements(arr: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let n = arr.len();
    let mut result: Vec<Complex<f64>> = arr.iter().skip(1).step_by(2).copied().collect();
    result.resize(n / 2, Complex::new(0.0, 0.0));
    result
}

pub fn gen_nth_root_of_unity(k: usize, n: usize, conjugate: bool) -> Complex<f64> {
    let angle: f64 = (2.0 * PI * k as f64) / n as f64;
    let result: Complex<f64> = Complex::new(round(angle.cos()), round(angle.sin()));
    if conjugate {
        return result.conj();
    }
    result
}

pub fn fft(mut arr: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let n = arr.len();

    if n == 1 {
        return arr
    }
    arr
}
