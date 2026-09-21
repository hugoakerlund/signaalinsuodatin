use crate::fft;
use std::f64::consts::PI;
use num::Complex;

pub struct Filter {
    lpf: Vec<Complex<f64>>,
    fft: fft::FFT,
    samples: Vec<i32>,
}

impl Filter {
    // Konstruktori luo alipäästösuodattimen ja saa argumentteina ääniraidan pituuden, johon suodatinta
    // sovelletaan, sen näytteenottotaajuuden sekä ylärajataajuuden.
    pub fn new(cutoff_frequency: f64, sample_rate: f64, samples: Vec<i32>) -> Self {

        let length = samples.len();
        let fft = fft::FFT::new(length);

        println!("\n1/4: Creating filter.");
        let supp = Self::create_supp(length);
        let ideal = Self::create_ideal(supp, cutoff_frequency, sample_rate);

        // Luodaan Hamming-ikkuna, jonka koko vastaa halutun suodattimen kokoa.
        let hamming: Vec<Complex<f64>> = Self::create_hamming_window(length);

        // Suodatin saadaan kertomalla taulukot keskenään.
        let mut filter: Vec<Complex<f64>> = Self::convolve_signals(hamming, ideal);

        // Suodatin muunnetaan aikatasosta taajuustasoon FFT-algoritmilla.
        println!("\t-Converting filter to frequency domain with FFT.");
        // let filter = fft::fft(filter, false);
        let lpf = fft.fft(&mut filter, false);
        println!("\t-Filter created.");

        Self {
            lpf: lpf,
            fft: fft,
            samples: samples,
        }
    }

    pub fn create_supp(length: usize) -> Vec<f64> {
        let mut supp: Vec<f64> = std::vec::from_elem(0.0, length);
        supp[0] = -(length as f64 - 1.0) / 2.0;
        for i in 1 .. length {
            supp[i] = supp[i - 1] + 1.0;
        }
        supp
    }

    pub fn create_ideal(supp: Vec<f64>, cutoff_frequency: f64, sample_rate: f64) -> Vec<Complex<f64>> {
        let length = supp.len();
        let mut ideal: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), length);
        for i in 0.. supp.len() {
            let cutoff = cutoff_frequency / sample_rate;
            let x = (2.0 * cutoff) * Self::sinc(2.0 * cutoff * supp[i]);
            ideal[i] = Complex::new(x, 0.0);
        }
        ideal
    }

    pub fn get_lpf(&self) -> Vec<Complex<f64>> {
        self.lpf.clone()
    }

    pub fn get_filtered_samples(&self) -> Vec<i32> {
        let frequency_samples: Vec<Complex<f64>> = Self::create_frequency_samples(&self);
        let filtered_samples: Vec<i32> = Self::apply_filter(&self, frequency_samples);
        filtered_samples
    }

    pub fn create_frequency_samples(&self) -> Vec<Complex<f64>> {

        // Ääninäytteet muunnetaan kompleksinumeroiksi.
        println!("\n2/4: Converting samples.");
        println!("\t-Converting samples to complex numbers.");
        let mut complex_samples = Self::convert_to_complex_samples(self.samples.clone());

        // Ääninäytteet muunnetaan aikatasosta taajuustasoon FFT-algoritmilla.
        println!("\t-Converting samples to frequency domain with FFT.");
        let frequency_samples = self.fft.fft(&mut complex_samples, false);
        println!("\t-Samples converted.");
        frequency_samples
    }

    pub fn apply_filter(&self, frequency_samples: Vec<Complex<f64>>) -> Vec<i32> {

        // Suodatin ja ääninäytteet ovat nyt taajustasossa. Niiden konvoluutio aikatasossa vastaa
        // kertomista taajuustasossa.
        println!("\n3/4: Applying filter on samples.");
        let mut filtered = Self::convolve_signals(frequency_samples, self.lpf.clone());
        println!("\t-Filter applied.");

        // Suodatetut ääninäytteet muunnetaan takaisin taajuustasosta aikatasoon.
        println!("\t-Converting filtered samples to time domain with IFFT.");
        let filtered_samples = self.fft.fft(&mut filtered, true);

        // Suodatetut ääninäytteet muunnetaan kompleksinumeroista takaisin kokonaisluvuiksi.
        println!("\t-Converting samples back to real numbers.");
        let mut new_samples = Self::convert_to_real_samples(filtered_samples);

        // Ääninäytteet ovat siirtyneet suodattimen käytön jälkeen (alkuperäisten ääninäytteiden pituus / 2)
        // askelta oikealle, joten nyt ne siirrettään takaisin tämän verran vasemmalle.
        new_samples.rotate_left(self.samples.len() / 2);

        // Ääninäytteet täytettiin nollilla, jotta niille voitiin tehdä Fourier-muunnos. Nyt nollat
        // poistetaan taulukon lopusta, jolloin ääniraidan pituus säilyy ennallaan.
        println!("\t-Resizing samples back to original length.");
        new_samples.resize(self.samples.len(), 0);
        new_samples
    }

    // Funktio kertoo keskenään kaksi kompleksiluvuista koostuvaa taulukkoa.
    pub fn convolve_signals(arr1: Vec<Complex<f64>>, arr2: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
        let n = arr1.len();
        let mut result: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), n);
        for i in 0 .. n {
            result[i] = arr1[i] * arr2[i];
        }
        result
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

    // Funktio muuntaa taulukon alkiot kokonaisluvuista kompleksiluvuiksi.
    pub fn convert_to_complex_samples(arr: Vec<i32>) -> Vec<Complex<f64>> {
        let n = arr.len();
        let mut result: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), n);
        for i in 0 .. n {
            result[i] = Complex::new(arr[i] as f64, 0.0);
        }
        result
    }

    // Funktio muuntaa taulukon alkiot kompleksiluvuista kokonaisluvuiksi.
    pub fn convert_to_real_samples(arr: Vec<Complex<f64>>) -> Vec<i32> {
        let n = arr.len();
        let mut result: Vec<i32> = std::vec::from_elem(0, n);
        for i in 0 .. n {
            let num = arr[i].re as i32;
            result[i] = num;
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
}

