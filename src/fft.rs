use std::f64::consts::PI;
use num::complex::Complex;

/// Tietue edustaa FFT-algoritmia.
pub struct FFT {

    /// FFT-algoritmin koko on luonnollinen luku ja se määrittelee pituuden taulukoille, joita
    /// algoritmi käsittelee.
    pub size: usize
}

impl FFT {

    /// Konstruktori luo uuden FFT:n argumenttina annetulle taulukon koolle. Mikäli koko ei ole
    /// kahden potenssi se asetetaan seuraavaan kahden potenssiin.
    ///
    /// # Esimerkit
    /// ```
    /// use num::complex::Complex;
    /// let mut arr: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(1.0, 0.0), 100);
    /// let fft = signaalinsuodatin::fft::FFT::new(arr.len());
    /// let result = fft.fft(&mut arr);
    ///
    /// assert_eq!(128, arr.len());
    /// ```
    pub fn new(size: usize) -> Self {
        let size = if size.is_power_of_two() { size } else { size.next_power_of_two() };
        Self {
            size: size
        }
    }

    /// Metodi palauttaa taulukon Fourier-muunnoksen. Taulukon pituus varmistetaan oikeaksi
    /// täyttämällä se nollilla FFT:n pituuteen.
    pub fn fft(&self, arr: &mut Vec<Complex<f64>>) -> Vec<Complex<f64>> {
        self.pad_with_zeros(arr);
        return self.radix_2_dit(arr.to_vec(), false);
    }

    /// Metodi palauttaa taulukon käänteisen Fourier-muunnoksen. Taulukon pituus varmistetaan
    /// oikeaksi täyttämällä se nollilla FFT:n pituuteen.
    pub fn ifft(&self, arr: &mut Vec<Complex<f64>>) -> Vec<Complex<f64>> {
        self.pad_with_zeros(arr);
        return self.radix_2_dit(arr.to_vec(), true);
    }

    /// Metodi täyttää taulukon nollilla FFT:n pituuteen, jotta taulukolle voidaan tehdä Fourier-muunnos.
    pub fn pad_with_zeros(&self, arr: &mut Vec<Complex<f64>>) {
        arr.resize(self.size, Complex::new(0.0, 0.0));
    }

    /// Metodi toteuttaa rekursiivisen Cooley-Tukey FFT-algoritmin ja palauttaa tuloksen uutena
    /// taulukkona. Metodi laskee tarvittaessa myös käänteisen Fourier-muunnoksen. Algoritmin
    /// aikavaativuus on O(n log n).
    fn radix_2_dit(&self, arr: Vec<Complex<f64>>, inverse: bool) -> Vec<Complex<f64>> {
        let n = arr.len();

        // Ehto, joka lopettaa rekursion.
        if n == 1 {
            return arr
        }

        // Syöte jaetaan kahteen osaan: parillisissa ja parittomissa indekseissä oleviin alkioihin.
        // Uuden syötteen koko on n/2, mikä rajoittaa algoritmin toimimisen syötteille, joiden koko on
        // 2^n. Usein tämä ei kuitenkaan ole tärkeä rajoitus.
        let even_elements: Vec<Complex<f64>> = self.get_even_elements(&arr);
        let odd_elements: Vec<Complex<f64>> = self.get_odd_elements(&arr);

        // Rekursiiviset funktiokutsut tehdään molemmille taulukoille.
        let even: Vec<Complex<f64>> = self.radix_2_dit(even_elements, inverse);
        let odd: Vec<Complex<f64>> = self.radix_2_dit(odd_elements, inverse);


        // Luodaan taulukko, johon lopuksi saadut tulokset yhdistetään.
        let mut combined: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), n);

        // Lasketaan yksikköjuuret. FFT-algoritmi käyttää yksikköjuuren kompleksikonjugaattia ja
        // käänteinen FFT-algoritmi ei käytä, minkä takia kolmannen argumentin edessä on negaatio.
        // Ensimmäinen yksikköjuuri on aina 1 ja sitä päivitetään silmukassa. Tämä säästää aikaa
        // laskennassa, kun juurta ei tarvitse laskea tyhjästä jokaisella silmukan kierroksella.
        let mut root: Complex<f64> = Complex::new(1.0, 0.0);
        let nth_root: Complex<f64> = self.get_nth_root_of_unity(n, !inverse);

        for k in 0 .. (n / 2) {


            // Lasketaan Fourier-muunnokset. Tämän FFT-algoritmin ydin on, että se laskee kaksi n/2
            // kokoista DFT:tä. Saatuja välituloksia käytetään myöhemmin uudelleen, mikä nopeuttaa
            // algoritmin toimintaa.
            let mut first_half: Complex<f64> = even[k] + root * odd[k];
            let mut second_half: Complex<f64> = even[k] - root * odd[k];

            // Käänteisessä DFT:ssä tulos kerrotaan luvulla (1/n). Kerromme luvulla (1/2) ja saamme
            // lopussa vastaavan tuloksen, koska välituloksia käytetään uudelleen log(2,n) kertaa,
            // jolloin lopussa tulos vastaa kertomista luvulla (1/n).
            if inverse {
                first_half *= Complex::<f64>::from(0.5);
                second_half *= Complex::<f64>::from(0.5);
            }

            // Yhdistetään saadut tulokset. Tästä käytetään myös nimitystä perhosdiagrammi, koska
            // operaation tiedonsiirtokaaviossa ristiin kulkevat nuolet muistuttavat perhosta.
            combined[k] = first_half;
            combined[k + n / 2] = second_half;

            // Päivitetään yksikköjuuri.
            root *= nth_root;
        }
        combined
    }

    /// Metodi palauttaa taulukon alkiot, joiden indeksi on parillinen.
    pub fn get_even_elements(&self, arr: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
        return arr.iter().step_by(2).copied().collect();
    }

    /// Metodi palauttaa taulukon alkiot, joiden indeksi on pariton.
    pub fn get_odd_elements(&self, arr: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
        return arr.iter().skip(1).step_by(2).copied().collect();
    }

    /// Metodi palauttaa n:nnen yksikköjuuren sekä tarvittaessa myös tämän juuren
    /// kompleksikonjugaatin.
    pub fn get_nth_root_of_unity(&self, n: usize, conjugate: bool) -> Complex<f64> {
        let angle: f64 = (2.0 * PI as f64) / n as f64;
        let result: Complex<f64> = Complex::new(angle.cos(), angle.sin());
        if conjugate {
            return result.conj();
        }
        result
    }

    /// Metodi palauttaa FFT-taulukon taajuuskorit. FFT-taulukko on symmetrinen, joten vain sen
    /// toisella puoliskolla on merkitystä. Tulostaulukon indeksiin i lasketaan taajuus, joka vastaa
    /// FFT-taulukon samaa indeksiä.
    pub fn get_frequency_bins(&self, sample_rate: f64) -> Vec<f64> {
        let length: usize = self.size / 2;
        let mut result: Vec<f64> = std::vec::from_elem(0.0, length);
        for i in 0 .. length {
            result[i] = i as f64 * sample_rate / self.size as f64;
        }
        result
    }

    /// Metodi palauttaa taajuuksia vastaavat voimakkuudet. FFT-taulukko on symmetrinen, joten vain
    /// sen toisella puoliskolla on merkitystä. Metodin laskemat voimakkuudet vastaavat
    /// taajuuskorien taajuuksia.
    pub fn get_frequency_magnitudes(&self, arr: Vec<Complex<f64>>) -> Vec<f64> {
        let mut result: Vec<f64> = std::vec::from_elem(0.0, self.size / 2);
        for i in 0 .. self.size / 2 {
            let num = arr[i].norm();
            result[i] = num / self.size as f64;
        }
        result
    }
}
