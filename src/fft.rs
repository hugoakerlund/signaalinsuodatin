use std::f64::consts::PI;
use num::complex::Complex;

// Funktio toteuttaa rekursiivisen Cooley-Tukey FFT-algoritmin ja palauttaa
// tuloksen uutena taulukkona. Funktio voi laskea myös käänteisen Fourier-muunnoksen. Algoritmin
// aikavaativuus on O(n log n).
pub fn fft(arr: Vec<Complex<f64>>, inverse: bool) -> Vec<Complex<f64>> {
    let n = arr.len();

    // Ehto, joka lopettaa rekursion.
    if n == 1 {
        return arr
    }

    // Syöte jaetaan kahteen osaan: parillisissa ja parittomissa indekseissä oleviin alkioihin.
    // Uuden syötteen koko on n/2, mikä rajoittaa algoritmin toimimisen syötteille, joiden koko on
    // 2^n. Usein tämä ei kuitenkaan ole tärkeä rajoitus.
    let even_elements: Vec<Complex<f64>> = get_even_elements(&arr);
    let odd_elements: Vec<Complex<f64>> = get_odd_elements(&arr);

    println!{"even {:?}", even_elements};
    println!{"odd {:?}", odd_elements};

    // Rekursiiviset funktiokutsut tehdään molemmille taulukoille.
    let even: Vec<Complex<f64>> = fft(even_elements, inverse);
    let odd: Vec<Complex<f64>> = fft(odd_elements, inverse);


    // Luodaan taulukko, johon lopuksi saadut tulokset yhdistetään.
    let mut combined: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), n);

    for k in 0 .. (n / 2) {

        // Lasketaan yksikköjuuri. FFT-algoritmi käyttää yksikköjuuren kompleksikonjugaattia ja
        // käänteinen FFT-algoritmi ei käytä, minkä takia kolmannen argumentin edessä on negaatio.
        let root: Complex<f64> = gen_nth_root_of_unity(k, n, !inverse);

        println!("\nw = {}", root);
        println!("k = {}", k);
        println!("even {}", even[k]);
        println!("odd {}", odd[k]);

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

// Funktio palauttaa taulukon alkiot, joiden indeksi on parillinen.
pub fn get_even_elements(arr: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let n = arr.len();
    let mut result: Vec<Complex<f64>> = arr.iter().step_by(2).copied().collect();
    result.resize(n / 2, Complex::new(0.0, 0.0));
    result
}

// Funktio palauttaa taulukon alkiot, joiden indeksi on pariton.
pub fn get_odd_elements(arr: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let n = arr.len();
    let mut result: Vec<Complex<f64>> = arr.iter().skip(1).step_by(2).copied().collect();
    result.resize(n / 2, Complex::new(0.0, 0.0));
    result
}

// Funktio palauttaa n:nnen yksikköjuuren, jossa k = 0 ... n. Funktio voi palauttaa myös tämän
// juuren kompleksikonjugaatin.
pub fn gen_nth_root_of_unity(k: usize, n: usize, conjugate: bool) -> Complex<f64> {
    let angle: f64 = (2.0 * PI * k as f64) / n as f64;
    let result: Complex<f64> = Complex::new(angle.cos(), angle.sin());
    if conjugate {
        return result.conj();
    }
    result
}

