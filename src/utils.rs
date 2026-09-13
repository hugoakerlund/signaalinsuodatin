use num::complex::Complex;

pub const ROUND_TO_DECIMALS: f64 = 1e10;

// Funktio pyöristää kompleksiluvun.
pub fn round(num: Complex<f64>) -> Complex<f64> {
    let rounded_re: f64 = (num.re * ROUND_TO_DECIMALS).round();
    let rounded_im: f64 = (num.im * ROUND_TO_DECIMALS).round();
    return Complex::new(rounded_re / ROUND_TO_DECIMALS, rounded_im / ROUND_TO_DECIMALS);
}

// Funktio pyöristää taulukossa olevat kompleksiluvut.
pub fn round_array(arr: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let n: usize = arr.len();
    let mut result: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), n);
    for i in 0 .. n {
        result[i] = round(arr[i]);
    }
    result
}

// Funktio laajentaa taulukon koon seuraavaan kahden potensiin ja täyttää nollilla uudet alkiot
// taulukon lopussa.
pub fn pad_with_zeros(arr: &mut Vec<Complex<f64>>) {
    let n: usize = arr.len();
    if n == 1 {
        return;
    }
    let mut new_size: usize = 2;
    while new_size < n {
        new_size *= 2;
    }
    arr.resize(new_size, Complex::new(0.0, 0.0));
}

// Muuntaa taulukon alkiot kokonaisluvuista kompleksiluvuiksi.
pub fn convert_to_complex_vector(arr: Vec<i32>) -> Vec<Complex<f64>> {
    let n = arr.len();
    let mut result: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), n);
    for i in 0 .. n {
        result[i] = Complex::new(arr[i] as f64, 0.0);
    }
    result
}

// Muuntaa taulukon alkiot kompleksiluvuista kokonaisluvuiksi.
pub fn convert_to_real_vector(arr: Vec<Complex<f64>>) -> Vec<i32> {
    let n = arr.len();
    let mut result: Vec<i32> = std::vec::from_elem(0, n);
    for i in 0 .. n {
        let num = arr[i].re as i32;
        result[i] = num;
    }
    result
}

