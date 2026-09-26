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
    /// let result = fft.fft(&mut arr, false);
    ///
    /// assert_eq!(128, arr.len());
    /// ```
    pub fn new(size: usize) -> Self {
        let size = if size.is_power_of_two() { size } else { size.next_power_of_two() };
        Self {
            size: size
        }
    }

    /// Metodi varmistaa, että taulukko on oikean kokoinen täyttämällä sen nollilla FFT:n pituuteen.
    /// Tämän jälkeen taulukolle tehdään Fourier-muunnos ja se palautetaan.
    pub fn fft(&self, arr: &mut Vec<Complex<f64>>, inverse: bool) -> Vec<Complex<f64>> {
        self.pad_with_zeros(arr);
        return self.radix_2_dit(arr.to_vec(), inverse);
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

        for k in 0 .. (n / 2) {

            // Lasketaan yksikköjuuri. FFT-algoritmi käyttää yksikköjuuren kompleksikonjugaattia ja
            // käänteinen FFT-algoritmi ei käytä, minkä takia kolmannen argumentin edessä on negaatio.
            let root: Complex<f64> = self.gen_nth_root_of_unity(k, n, !inverse);

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

    /// Metodi palauttaa n:nnen yksikköjuuren, jossa k = 0 ... n. Metodi palautaa tarvittaessa myös
    /// tämän juuren kompleksikonjugaatin.
    pub fn gen_nth_root_of_unity(&self, k: usize, n: usize, conjugate: bool) -> Complex<f64> {
        let angle: f64 = (2.0 * PI * k as f64) / n as f64;
        let result: Complex<f64> = Complex::new(angle.cos(), angle.sin());
        if conjugate {
            return result.conj();
        }
        result
    }
}
