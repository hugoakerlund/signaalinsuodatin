use num::complex::Complex;
use std::f64::consts::PI;
use crate::{io, fft, filter, wav_filter};

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
/// assert!(signaalinsuodatin::utils::fft_array_is_symmetrical(arr));
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

/// Funktio luo annetulle pituudelle, voimakkuudelle ja näytteenottotaajuudelle signaalin, joka
/// sisältää taulukossa olevat taajuudet. Signaali saadaan luotua yksinkertaisesti summaamalla
/// taajuudet yhteen taulukkoon.
pub fn generate_signal_of_frequencies(frequencies: Vec<f64>, strength: f64, sample_rate: f64, lenght: usize) -> Vec<i32> {
    let mut result: Vec<i32> = std::vec::from_elem(0, lenght);
    for freq in frequencies {
        let phase_inc: f64 = 2.0 * PI * (freq / sample_rate);
        let mut phase: f64 = 0.0;

        for i in 0 .. lenght {
            result[i] += (phase.sin() * strength) as i32;
            phase += phase_inc;
        }
    }
    result
}


/// Funktio palauttaa ylärajataajuuden yläpuolella olevan osuuden taajuuksien voimakkuuksista
/// prosentteina.
pub fn get_filtered_percentage_over_cutoff(file_name: &str, cutoff_frequency: f64) -> f64 {
    let io = io::IO::new();
    let file_to_read = io.read_input_file(file_name);
    let file_to_filter = io.read_input_file(file_name);

    let spec = file_to_read.spec();
    let channels = spec.channels;
    let sample_rate = spec.sample_rate as f64;

    let wav_filter = wav_filter::WAVFilter::new(file_to_filter, cutoff_frequency);
    let mut filtered_samples = wav_filter.get_filtered_samples();

    if channels == 2 {
        let (left_filtered, _right_filtered): (Vec<i32>, Vec<i32>) = wav_filter::WAVFilter::extract_left_and_right_channels(filtered_samples.clone());
        filtered_samples = left_filtered;
    }

    let magnitudes: (f64, f64) = get_magnitudes(filtered_samples.clone(), cutoff_frequency, sample_rate);

    magnitudes.1 / (magnitudes.0 + magnitudes.1)
}


/// Funktio palauttaa taajuuksien voimakkuudet sekä ylärajataajuuden alapuolella että sen
/// yläpuolella. Ääninäytteet ikkunoidaan ja niille tehdään Fourier-muunnos. Tämän jälkeen
/// taajuuskorit ja niitä vastaavat voimakkuudet lasketaan. Niiden avulla saadaan summattua
/// voimakkuudet ylärajataajuuden alapuolella ja yläpuolella.
pub fn get_magnitudes(samples: Vec<i32>, cutoff_frequency: f64, sample_rate: f64) -> (f64, f64) {
    let fft = fft::FFT::new(samples.len());
    let length = samples.len();

    let complex_samples = filter::Filter::convert_to_complex_samples(samples);
    let hamming: Vec<Complex<f64>> = filter::Filter::create_hamming_window(length);
    let mut windowed_samples = filter::Filter::convolve_signals(hamming, complex_samples);
    let sample_frequencies = fft.fft(&mut windowed_samples);

    let magnitudes: Vec<f64> = fft.get_frequency_magnitudes(sample_frequencies);
    let frequency_bins = fft.get_frequency_bins(sample_rate);

    return sum_magnitudes_under_and_over_cutoff(frequency_bins, cutoff_frequency, magnitudes);
}

/// Funktio summaa taajuuksien voimakkuudet sekä ylärajataajuuden alapuolella että sen yläpuolella.
/// Tulos palautetaan kaksikkona, jossa ensimmäinen vastaa ylärajataajuuden alapuolella olevien
/// voimakkuuksien summaa ja toinen sen yläpuolella olevien voimakkuuksien summaa.
pub fn sum_magnitudes_under_and_over_cutoff(frequency_bins: Vec<f64>, cutoff_frequency: f64, magnitudes: Vec<f64>) -> (f64, f64) {
    let mut under_cutoff: f64 = 0.0;
    let mut over_cutoff: f64 = 0.0;
    let length = frequency_bins.len();
    let mut i: usize = 0;

    while i < length {

        // Summattaan voimakkuudet ylärajataajuuden yläpuolella, jos indeksi vastaa ylärajataajuutta
        // suurempia taajuuksia.
        if cutoff_frequency <= frequency_bins[i] {
            over_cutoff += magnitudes[i];
        }

        // Summattaan voimakkuudet ylärajataajuuden alapuolella, jos indeksi vastaa ylärajataajuutta
        // pienempiä taajuuksia.
        else {
            under_cutoff += magnitudes[i];
        }
        i += 1;
    }

    (under_cutoff, over_cutoff)
}
