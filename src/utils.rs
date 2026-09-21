use num::complex::Complex;

pub const ROUND_TO_DECIMALS: f64 = 1e10;

// Funktio pyöristää kompleksiluvun.
pub fn round_c64(num: Complex<f64>) -> Complex<f64> {
    let rounded_re: f64 = (num.re * ROUND_TO_DECIMALS).round();
    let rounded_im: f64 = (num.im * ROUND_TO_DECIMALS).round();
    return Complex::new(rounded_re / ROUND_TO_DECIMALS, rounded_im / ROUND_TO_DECIMALS);
}

// Funktio pyöristää liukuluvun.
pub fn round_f64(num: f64) -> f64 {
    return (num * ROUND_TO_DECIMALS).round() / ROUND_TO_DECIMALS;
}

// Funktio testaa FFT-algoritmin tuottaman taulukon symmetrisyyttä.
pub fn fft_array_is_symmetrical(arr: Vec<Complex<f64>>) -> bool {
    let n = arr.len();
    for i in 1 .. n {
        let left: Complex<f64> = arr[i];
        let right: Complex<f64> = arr[n - i];
        if round_c64(left) != round_c64(right.conj()) {
            return false;
        }
    }
    return true;
}

// Funktio pyöristää taulukossa olevat kompleksiluvut.
pub fn round_array(arr: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let n: usize = arr.len();
    let mut result: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), n);
    for i in 0 .. n {
        result[i] = round_c64(arr[i]);
    }
    result
}
