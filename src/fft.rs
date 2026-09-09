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

pub fn fft(arr: Vec<Complex<f64>>, inverse: bool) -> Vec<Complex<f64>> {
    let n = arr.len();

    if n == 1 {
        return arr
    }

    let even_elements: Vec<Complex<f64>> = get_even_elements(&arr);
    let odd_elements: Vec<Complex<f64>> = get_odd_elements(&arr);

    println!{"even {:?}", even_elements};
    println!{"odd {:?}", odd_elements};

    let even: Vec<Complex<f64>> = fft(even_elements, inverse);
    let odd: Vec<Complex<f64>> = fft(odd_elements, inverse);


    let mut combined: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), n);

    for k in 0 .. (n / 2) {

        let root: Complex<f64> = gen_nth_root_of_unity(k, n, !inverse);

        println!("\nw = {}", root);
        println!("k = {}", k);
        println!("even {}", even[k]);
        println!("odd {}", odd[k]);

        let mut first_half: Complex<f64> = even[k] + root * odd[k];
        let mut second_half: Complex<f64> = even[k] - root * odd[k];

        if inverse {
            first_half *= Complex::<f64>::from(0.5);
            second_half *= Complex::<f64>::from(0.5);
        }

        combined[k] = first_half;
        combined[k + n / 2] = second_half;
    }
    combined
}
