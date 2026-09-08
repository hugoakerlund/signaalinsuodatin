use std::f64::consts::PI;
use num::complex::Complex;

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

pub fn gen_nth_roots_of_unity(n: usize, conjugate: bool) -> Vec<Complex<f64>> {
    let mut result: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), n);
    let coefficient: f64 = if conjugate {-2.0} else {2.0};
    for k in 0 .. n {
        let angle: f64 = (coefficient * PI * k as f64) / n as f64;
        let real: f64 = (angle.cos() * 1000000.0).round() / 1000000.0;
        let img: f64 = (angle.sin() * 1000000.0).round() / 1000000.0;
        result[k] = Complex::new(real, img);
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
