use crate::{utils, fft};
use num::Complex;

pub struct Filter {
    lpf: Vec<Complex<f64>>,
    samples: Vec<i32>,
}

impl Filter {
    pub fn new(cutoff_frequency: f64, sample_rate: f64, samples: Vec<i32>) -> Self {
        let lpf = Self::create_low_pass_filter(samples.len() + 1, sample_rate, cutoff_frequency);

        Self {
            lpf: lpf,
            samples: samples,
        }
    }

    pub fn get_lpf(&self) -> Vec<Complex<f64>> {
        self.lpf.clone()
    }

    pub fn get_filtered_samples(&self) -> Vec<i32> {
        let prepared_samples: Vec<Complex<f64>> = Self::create_prepared_samples(self.samples.clone());
        let filtered_samples: Vec<i32> = Self::apply_filter(prepared_samples, self.lpf.clone(), self.samples.len());
        filtered_samples
    }

    fn create_low_pass_filter(length: usize, sample_rate: f64, cutoff_frequency: f64) -> Vec<Complex<f64>> {
        println!("\n1/4: Creating filter.");

        let mut supp: Vec<f64> = std::vec::from_elem(0.0, length);
        supp[0] = -(length as f64 - 1.0) / 2.0;
        for i in 1 .. length {
            supp[i] = supp[i - 1] + 1.0;
        }

        // https://en.wikipedia.org/wiki/Sinc_filter#Sinc-in-time
        let mut ideal: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), length);
        for i in 0.. length {
            let x = (2.0 * cutoff_frequency / sample_rate) * utils::sinc(2.0 * cutoff_frequency * supp[i] / sample_rate);
            ideal[i] = Complex::new(x, 0.0);
        }

        let hamming: Vec<Complex<f64>> = utils::create_hamming_window(length);
        let mut filter: Vec<Complex<f64>> = utils::multiply_complex_arrays(hamming, ideal);

        // Suodatin täytetään nollilla seuraavan kahden potenssin pituuteen, jotta sille voidaan
        // tehdä Fourier-muunnos.
        println!("\t-Padding filter with zeros.");
        utils::pad_with_zeros(&mut filter);
        println!("\t-Size after padding with zeros: {}.", filter.len());

        println!("\t-Performing FFT on filter.");
        let filter = fft::fft(filter, false);
        println!("\t-Filter created.");
        filter
    }

    pub fn create_prepared_samples(samples: Vec<i32>) -> Vec<Complex<f64>> {
        println!("\n2/4: Preparing samples.");
        println!("\t-Converting samples to complex array.");
        let mut complex_samples = utils::convert_to_complex_array(samples);

        // Ääninäytteet täytetään nollilla seuraavan kahden potenssin pituuteen, jotta sille voidaan
        // tehdä Fourier-muunnos.
        println!("\t-Padding samples with zeros.");
        utils::pad_with_zeros(&mut complex_samples);
        println!("\t-Size after padding with zeros: {}.", complex_samples.len());

        println!("\t-Performing FFT on samples.");
        let prepared_samples = fft::fft(complex_samples, false);
        println!("\t-Samples prepared.");
        prepared_samples
    }

    pub fn apply_filter(prepared_samples: Vec<Complex<f64>>, filter: Vec<Complex<f64>>, original_length: usize) -> Vec<i32> {
        println!("\n3/4: Applying filter on samples.");
        let filtered = utils::multiply_complex_arrays(prepared_samples, filter);
        println!("\t-Filter applied.");

        println!("\t-Performing IFFT on filtered samples.");
        let processed_samples = fft::fft(filtered, true);

        println!("\t-Converting samples back to real array.");
        let mut new_samples = utils::convert_to_real_array(processed_samples);

        // Ääninäytteet ovat siirtyneet suodattimen käytön jälkeen (alkuperäisten ääninäytteiden pituus / 2)
        // askelta oikealle, joten nyt ne siirrettään takaisin tämän verran vasemmalle.
        new_samples.rotate_left(original_length / 2);

        // Ääninäytteet täytettiin nollilla, jotta niille voitiin tehdä Fourier-muunnos. Nyt nollat
        // poistetaan taulukon lopusta, jolloin ääniraidan pituus säilyy ennallaan.
        println!("\t-Resizing samples back to original length.");
        new_samples.resize(original_length, 0);
        new_samples
    }
}

