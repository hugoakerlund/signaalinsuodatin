use crate::fft;
use std::f64::consts::PI;
use num::Complex;

/// Tietue edustaa digitaalista suodatinta.
pub struct Filter {
    /// Alipäästösuodatin on kompleksinumeroista koostuva taulukko.
    pub lpf: Vec<Complex<f64>>,
    ///  FFT-oliota käytetään aikatason ja taajuustason välillä tapahtuvaan muuntamiseen.
    pub fft: fft::FFT,
    /// Ääninäytteet ovat kokonaisluvuista koostuva taulukko.
    pub samples: Vec<i32>,
}

impl Filter {
    /// Konstruktori luo alipäästösuodattimen ja saa argumentteina ääniraidan pituuden, sen sämpläys
    /// taajuuden sekä ylärajataajuuden, jota korkeammat taajuudet poistetaan.
    ///
    /// # Esimerkit
    /// ```
    /// let samples: Vec<i32> = std::vec::from_elem(1000, 100);
    /// let cutoff_frequency: f64 = 500.0;
    /// let sample_rate: f64 = 44100.0;
    ///
    /// let filter = signaalinsuodatin::filter::Filter::new(cutoff_frequency, sample_rate, samples);
    /// let filtered_samples = filter.get_filtered_samples();
    ///
    /// assert_eq!(100, filtered_samples.len());
    /// ```
    pub fn new(cutoff_frequency: f64, sample_rate: f64, samples: Vec<i32>) -> Self {

        // Ääninäytteiden pituutta tarvitaan oikean kokoisen suodattimen luomiseen.
        let length = samples.len();

        // Luodaan FFT-olio, joka käsittelee ääninäytteiden pituuden mukaisia taulukoita.
        let fft = fft::FFT::new(length);

        // Lasketaan idealisoitu alipäästösuodatin annetun ylärajan ja ääninäytteiden perusteella.
        println!("\n1/4: Creating filter.");
        let coefficients = Self::create_coefficients(length + 1);
        let ideal = Self::create_ideal(coefficients, cutoff_frequency, sample_rate);

        // Luodaan Hamming-ikkuna, jonka koko vastaa halutun suodattimen kokoa.
        let hamming: Vec<Complex<f64>> = Self::create_hamming_window(length);

        // Suodatin saadaan kertomalla taulukot keskenään.
        let mut filter: Vec<Complex<f64>> = Self::convolve_signals(hamming, ideal);

        // Suodatin muunnetaan aikatasosta taajuustasoon FFT-algoritmilla.
        println!("\t-Converting filter to frequency domain with FFT.");
        let lpf = fft.fft(&mut filter, false);
        println!("\t-Filter created.");

        Self {
            lpf: lpf,
            fft: fft,
            samples: samples,
        }
    }

    /// Funktio luo kertoimet, joita käytetään idealisoidun suodattimen luomiseen. Kertoimet ovat
    /// symmetriset y-akselin molemminpuolin.
    ///
    /// # Esimerkit
    ///
    /// ```
    /// let n: usize = 101;
    /// let result: Vec<f64> = signaalinsuodatin::filter::Filter::create_coefficients(n);
    /// assert_eq!(-50.0, result[0]);
    /// assert_eq!(50.0, result[n-1]);
    /// ```
    pub fn create_coefficients(length: usize) -> Vec<f64> {
        let mut coefficients : Vec<f64> = std::vec::from_elem(0.0, length);
        coefficients[0] = -(length as f64 - 1.0) / 2.0;
        for i in 1 .. length {
            coefficients[i] = coefficients[i - 1] + 1.0;
        }
        coefficients
    }

    /// Funktio laskee idealisoidun alipäästösuodattimen, joka poistaa kaikki taajuudet argumenttina
    /// annetun ylärajan yläpuolelta muuttamatta alempia taajuuksia. Suodatinta kutsutaankin tämän
    /// takia ns. tiiliseinäsuodattimeksi. Suodattimen vaihevaste on lineaarinen, minkä takia
    /// signaalin vaihe siirtyy kertoimien verran suodatuksen jälkeen.
    ///
    /// # Esimerkit
    ///
    /// ```
    /// use num::Complex;
    /// let n: usize = 101;
    /// let coefficients: Vec<f64> = signaalinsuodatin::filter::Filter::create_coefficients(n);
    ///
    /// let cutoff_frequency = 100.0;
    /// let sample_rate = 44100.0;
    /// let ideal: Vec<Complex<f64>> = signaalinsuodatin::filter::Filter::create_ideal(coefficients,
    /// cutoff_frequency, sample_rate);
    ///
    /// assert_eq!(ideal[0], ideal[n-1]);
    /// ```
    pub fn create_ideal(coefficients: Vec<f64>, cutoff_frequency: f64, sample_rate: f64) -> Vec<Complex<f64>> {
        let length = coefficients.len();
        let mut ideal: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), length);
        for i in 0.. coefficients.len() {
            let cutoff = cutoff_frequency / sample_rate;
            let x = (2.0 * cutoff) * Self::sinc(2.0 * cutoff * coefficients[i]);
            ideal[i] = Complex::new(x, 0.0);
        }
        ideal
    }

    /// Metodi palauttaa suodatetut ääninäytteet aikatasossa.
    pub fn get_filtered_samples(&self) -> Vec<i32> {
        let frequency_samples: Vec<Complex<f64>> = Self::create_frequency_samples(&self);
        let filtered_samples: Vec<i32> = Self::apply_filter(&self, frequency_samples);
        filtered_samples
    }

    /// Metodi muuntaan ääninäytteet aikatasosta taajuustasoon.
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

    /// Metodi soveltaa suodatinta taajuustasossa oleviin ääninäytteisiin, jotka se saa argumenttina.
    pub fn apply_filter(&self, frequency_samples: Vec<Complex<f64>>) -> Vec<i32> {

        // Suodatin ja ääninäytteet ovat nyt taajustasossa. Niiden konvoluutio aikatasossa vastaa
        // niiden kertomista keskenään taajuustasossa.
        println!("\n3/4: Applying filter on samples.");
        let mut filtered = Self::convolve_signals(frequency_samples, self.lpf.clone());
        println!("\t-Filter applied.");

        // Suodatetut ääninäytteet muunnetaan takaisin taajuustasosta aikatasoon.
        println!("\t-Converting filtered samples to time domain with IFFT.");
        let filtered_samples = self.fft.fft(&mut filtered, true);

        // Suodatetut ääninäytteet muunnetaan kompleksinumeroista takaisin kokonaisluvuiksi.
        println!("\t-Converting samples back to real numbers.");
        let mut new_samples = Self::convert_to_real_samples(filtered_samples);

        // Suodatettujen ääninäytteiden vaihe korjataan.
        self.correct_samples_phase(&mut new_samples);

        // Suodatettujen ääninäytteiden pituus korjataan.
        println!("\t-Resizing samples back to original length.");
        self.correct_samples_length(&mut new_samples);
        new_samples
    }

    /// Metodi korjaa ääninäytteiden vaiheen suodattimen käytön jälkeen. Ääninäytteet ovat
    /// viivästyneet suodattimen käytön jälkeen (alkuperäisten ääninäytteiden pituus / 2) askelta,
    /// joten niitä siirrettään takaisin tämän verran vasemmalle.
    fn correct_samples_phase(&self, arr: &mut Vec<i32>) {
        arr.rotate_left(self.samples.len() / 2);
    }

    /// Metodi korjaa ääninäytteiden pituuden takaisin alkuperäiseen. Ääninäytteet täytettiin
    /// nollilla, jotta niille voitiin tehdä Fourier-muunnos. Nollat poistetaan taulukon lopusta,
    /// jolloin ääniraidan pituus säilyy ennallaan.
    fn correct_samples_length(&self, arr: &mut Vec<i32>) {
        arr.resize(self.samples.len(), 0);
    }

    /// Funktio kertoo keskenään kaksi ääninäytettä. Taajuustasossa olevien ääninäytteiden kertominen
    /// keskenään vastaa niiden konvoluutiota aikatasossa.
    ///
    /// # Esimerkit
    ///
    /// ```
    /// use num::Complex;
    /// let signal1: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(2.0, 2.0), 10);
    /// let signal2: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(4.0, 4.0), 10);
    ///
    /// let result: Vec<Complex<f64>> = signaalinsuodatin::filter::Filter::convolve_signals(signal1, signal2);
    /// assert_eq!(Complex::new(0.0, 16.0), result[0]);
    /// ```
    pub fn convolve_signals(arr1: Vec<Complex<f64>>, arr2: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
        let n = arr1.len();
        let mut result: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), n);
        for i in 0 .. n {
            result[i] = arr1[i] * arr2[i];
        }
        result
    }

    /// Funktio luo Hamming-ikkunan. Ikkuna muistuttaa paljon Hann-ikkunaa, sillä sen muodon määrää
    /// kosinifunktio, mutta vakiot a_0 ja a_1 ovat eri. Ikkunaa käytetään suodattimen luomiseen.
    /// Digitaalisissa suodattimissa tätä kutsutaan ikkunametodiksi.
    ///
    /// # Esimerkit
    ///
    /// ```
    /// use num::Complex;
    /// let n: usize = 10;
    /// let result: Vec<Complex<f64>> = signaalinsuodatin::filter::Filter::create_hamming_window(n);
    /// assert_eq!(0.53836 - 0.46164, result[0].re);
    /// assert_eq!(0.9774057301824945, result[n-1].re);
    /// ```
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

    /// Funktio muuntaa ääninäytteet kokonaisluvuista kompleksiluvuiksi.
    ///
    /// # Esimerkit
    ///
    /// ```
    /// use num::Complex;
    /// let arr: Vec<i32> = std::vec::from_elem(1, 10);
    /// let result: Vec<Complex<f64>> = signaalinsuodatin::filter::Filter::convert_to_complex_samples(arr);
    /// assert_eq!(Complex::new(1.0, 0.0), result[0]);
    /// ```
    pub fn convert_to_complex_samples(arr: Vec<i32>) -> Vec<Complex<f64>> {
        let n = arr.len();
        let mut result: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), n);
        for i in 0 .. n {
            result[i] = Complex::new(arr[i] as f64, 0.0);
        }
        result
    }

    /// Funktio muuntaa ääninäyteet kompleksiluvuista kokonaisluvuiksi.
    ///
    /// # Esimerkit
    ///
    /// ```
    /// use num::Complex;
    /// let arr: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(1.0, 1.0), 10);
    /// let result: Vec<i32> = signaalinsuodatin::filter::Filter::convert_to_real_samples(arr);
    /// assert_eq!(1, result[0]);
    /// ```
    pub fn convert_to_real_samples(arr: Vec<Complex<f64>>) -> Vec<i32> {
        let n = arr.len();
        let mut result: Vec<i32> = std::vec::from_elem(0, n);
        for i in 0 .. n {
            let num = arr[i].re as i32;
            result[i] = num;
        }
        result
    }

    /// Funktio toteuttaa normalisoidun sinifunktion. Normalisoitua sinifunktiota käytetään
    /// usein signaalinkäsittelyssä. Erityisesti tässä projektissa sitä käytetään suodattimen luomiseen.
    ///
    /// # Esimerkit
    ///
    /// ```
    /// let result: f64 = signaalinsuodatin::filter::Filter::sinc(0.5);
    /// assert_eq!(0.6366197723675813430, result);
    /// ```
    pub fn sinc(x: f64) -> f64 {
        if x == 0.0 {
            return 1.0;
        }
        let pi_x: f64 = PI * x;
        return pi_x.sin() / pi_x;
    }
}
