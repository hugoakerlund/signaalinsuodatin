use std::f64::consts::E;
use std::f64::consts::PI;
use num::complex::Complex;

pub fn get_even_elements(arr: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let result: Vec<Complex<f64>> = arr.iter().skip(1).step_by(2).copied().collect();
    result
}

pub fn get_odd_elements(arr: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let result: Vec<Complex<f64>> = arr.iter().step_by(2).copied().collect();
    result
}

pub fn fft(mut arr: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let n = arr.len();

    if n == 1 {
        return arr
    }
    arr
}
