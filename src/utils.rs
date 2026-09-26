use num::complex::Complex;

pub const ROUND_TO_DECIMALS: f64 = 1e10;

/// Funktio pyöristää kompleksiluvun.
///
/// # Esimerkit
///
/// ```
/// use num::Complex;
/// let x: Complex<f64> = Complex::new(0.999999999999999, 0.999999999999999);
/// let result = signaalinsuodatin::utils::round_cmplx(x);
/// assert_eq!(Complex::new(1.0, 1.0) , result);
/// ```
pub fn round_cmplx(num: Complex<f64>) -> Complex<f64> {
    let rounded_re: f64 = (num.re * ROUND_TO_DECIMALS).round();
    let rounded_im: f64 = (num.im * ROUND_TO_DECIMALS).round();
    return Complex::new(rounded_re / ROUND_TO_DECIMALS, rounded_im / ROUND_TO_DECIMALS);
}

/// Funktio pyöristää liukuluvun.
///
/// # Esimerkit
///
/// ```
/// let x: f64 = 0.999999999999999;
/// let result = signaalinsuodatin::utils::round_float(x);
/// assert_eq!(1.0, result);
/// ```
pub fn round_float(num: f64) -> f64 {
    return (num * ROUND_TO_DECIMALS).round() / ROUND_TO_DECIMALS;
}

/// Funktio pyöristää taulukossa olevat kompleksiluvut.
///
/// # Esimerkit
///
/// ```
/// use num::Complex;
/// let arr: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.999999999999999, 0.999999999999999), 10);
/// let expected: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(1.0, 1.0), 10);
/// let result = signaalinsuodatin::utils::round_array(arr);
/// assert_eq!(expected, result);
/// ```
pub fn round_array(arr: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let n: usize = arr.len();
    let mut result: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), n);
    for i in 0 .. n {
        result[i] = round_cmplx(arr[i]);
    }
    result
}

/// Funktio testaa FFT-algoritmin tuottaman taulukon symmetrisyyttä. Algoritmin tuottaman taulukon
/// vasen puoli ensimmäisestä alkiosta eteenpäin vastaa aina sen oikean puolen
/// kompleksikonjungaattia.
///
/// # Esimerkit
///
/// ```
/// use num::Complex;
/// let arr: Vec<Complex<f64>> = vec![
///     Complex::new(10.0, 0.0),
///     Complex::new(-2.0, 2.0),
///     Complex::new(-2.0, 0.0),
///     Complex::new(-2.0, -2.0),
/// ];
/// assert_eq!(true, signaalinsuodatin::utils::fft_array_is_symmetrical(arr));
/// ```
pub fn fft_array_is_symmetrical(arr: Vec<Complex<f64>>) -> bool {
    let n = arr.len();
    for i in 1 .. n {
        let left: Complex<f64> = arr[i];
        let right: Complex<f64> = arr[n - i];
        if round_cmplx(left) != round_cmplx(right.conj()) {
            return false;
        }
    }
    return true;
}

