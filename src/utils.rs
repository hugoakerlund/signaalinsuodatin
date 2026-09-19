use num::complex::Complex;
use std::f64::consts::PI;

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

// Funktio pyöristää taulukossa olevat kompleksiluvut.
pub fn round_array(arr: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let n: usize = arr.len();
    let mut result: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), n);
    for i in 0 .. n {
        result[i] = round_c64(arr[i]);
    }
    result
}

// Funktio laajentaa taulukon koon seuraavaan kahden potensiin ja täyttää nollilla uudet alkiot
// taulukon lopussa.
pub fn pad_with_zeros(arr: &mut Vec<Complex<f64>>) {
    let n: usize = arr.len();
    let x = n.next_power_of_two();
    arr.resize(x, Complex::new(0.0, 0.0));
}

// Funktio muuntaa taulukon alkiot kokonaisluvuista kompleksiluvuiksi.
pub fn convert_to_complex_array(arr: Vec<i32>) -> Vec<Complex<f64>> {
    let n = arr.len();
    let mut result: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), n);
    for i in 0 .. n {
        result[i] = Complex::new(arr[i] as f64, 0.0);
    }
    result
}

// Funktio muuntaa taulukon alkiot kompleksiluvuista kokonaisluvuiksi.
pub fn convert_to_real_array(arr: Vec<Complex<f64>>) -> Vec<i32> {
    let n = arr.len();
    let mut result: Vec<i32> = std::vec::from_elem(0, n);
    for i in 0 .. n {
        let num = arr[i].re as i32;
        result[i] = num;
    }
    result
}

// Funktio kertoo keskenään kaksi kompleksiluvuista koostuvaa taulukkoa.
pub fn multiply_complex_arrays(arr1: Vec<Complex<f64>>, arr2: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let n = arr1.len();
    let mut result: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), n);
    for i in 0 .. n {
        result[i] = arr1[i] * arr2[i];
    }
    result
}

// Funktio toteuttaa normalisoidun sinifunktion toteutus. Normalisoitua sinifunktiota käytetään
// usein signaalinkäsittelyssä. Erityisesti tässä projektissa sitä käytetään suodattimen luomiseen.
pub fn sinc(x: f64) -> f64 {
    if x == 0.0 {
        return 1.0;
    }
    let pi_x: f64 = PI * x;
    return pi_x.sin() / pi_x;
}

// Funktio luo Hamming-ikkunan. Ikkuna muistuttaa paljon Hann-ikkunaa, sillä sen muodon määrää
// kosinifunktio, mutta vakiot a_0 ja a_1 ovat eri. Ikkunaa käytetään suodattimen luomiseen.
// Digitaalisissa suodattimissa tätä kutsutaan ikkunametodiksi.
pub fn create_hamming_window(length: usize) -> Vec<Complex<f64>> {
    let mut result: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), length);
    for i in 0 .. length {
        let a_0: f64 = 0.53836;
        let a_1: f64 = -0.46164;
        let x = (PI * i as f64) / length as f64;
        let res = a_0 + a_1 * x.cos();
        result[i] = Complex::new(res, 0.0);
    }
    result
}
