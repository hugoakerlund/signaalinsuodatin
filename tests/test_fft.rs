use signaalinsuodatin::{fft, utils};
use num::complex::Complex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn even_elements_are_collected() -> () {
        let fft = fft::FFT::new(0);

        let arr: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
            Complex::new(5.0, 0.0),
            Complex::new(6.0, 0.0),
            Complex::new(7.0, 0.0),
            Complex::new(8.0, 0.0),
        ];
        let expected: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(5.0, 0.0),
            Complex::new(7.0, 0.0),
        ];

        let result = fft.get_even_elements(&arr);
        assert_eq!(result, expected);

        let arr2: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
            Complex::new(5.0, 0.0),
        ];
        let expected2: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(5.0, 0.0),
        ];
        let result2 = fft.get_even_elements(&arr2);
        assert_eq!(result2, expected2);
    }

    #[test]
    fn odd_elements_are_collected() -> () {
        let fft = fft::FFT::new(0);

        let arr: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
            Complex::new(5.0, 0.0),
            Complex::new(6.0, 0.0),
            Complex::new(7.0, 0.0),
            Complex::new(8.0, 0.0),
        ];
        let expected: Vec<Complex<f64>> = vec![
            Complex::new(2.0, 0.0),
            Complex::new(4.0, 0.0),
            Complex::new(6.0, 0.0),
            Complex::new(8.0, 0.0),
        ];
        let result = fft.get_odd_elements(&arr);
        assert_eq!(result, expected);

        let arr2: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
            Complex::new(5.0, 0.0),
        ];
        let expected2: Vec<Complex<f64>> = vec![
            Complex::new(2.0, 0.0),
            Complex::new(4.0, 0.0),
        ];
        let result2 = fft.get_odd_elements(&arr2);
        assert_eq!(result2, expected2);
    }

    #[test]
    fn nth_roots_of_unity_are_generated_correctly() -> () {
        let fft = fft::FFT::new(0);

        let expected_roots: Vec<Complex<f64>> = vec![
            Complex::new( 1.0,           0.0),
            Complex::new(-1.0,           0.0),
            Complex::new(-0.5,           0.8660254038),
            Complex::new(0.0,            1.0),
            Complex::new(0.3090169944,   0.9510565163),
            Complex::new(0.5,            0.8660254038),
            Complex::new(0.6234898019,   0.7818314825),
            Complex::new(0.7071067812,   0.7071067812),
            Complex::new(0.7660444431,   0.6427876097),
            Complex::new(0.8090169944,   0.5877852523),
        ];

        let expected_conjugate_roots: Vec<Complex<f64>> = vec![
            Complex::new( 1.0,           0.0),
            Complex::new(-1.0,           0.0),
            Complex::new(-0.5,          -0.8660254038),
            Complex::new(0.0,           -1.0),
            Complex::new(0.3090169944,  -0.9510565163),
            Complex::new(0.5,           -0.8660254038),
            Complex::new(0.6234898019,  -0.7818314825),
            Complex::new(0.7071067812,  -0.7071067812),
            Complex::new(0.7660444431,  -0.6427876097),
            Complex::new(0.8090169944,  -0.5877852523),
        ];

        let length = expected_roots.len();
        let mut nth_roots: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), length);
        let mut nth_roots_conjugate: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), length);

        for n in 0 .. length {
            nth_roots[n] = fft.get_nth_root_of_unity(n + 1, false);
            nth_roots_conjugate[n] = fft.get_nth_root_of_unity(n + 1, true);
        }

        assert_eq!(utils::round_array(nth_roots), expected_roots);
        assert_eq!(utils::round_array(nth_roots_conjugate), expected_conjugate_roots);
    }

    #[test]
    fn fft_is_working() -> () {

        let arr: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
        ];
        let expected: Vec<Complex<f64>> = vec![
            Complex::new(3.0, 0.0),
            Complex::new(-1.0, 0.0),
        ];
        let fft = fft::FFT::new(arr.len());
        let result: Vec<Complex<f64>> = fft.fft(&mut arr.clone());
        assert_eq!(utils::round_array(result.clone()), expected);
        assert_eq!(utils::round_array(fft.ifft(&mut result.clone())), arr.clone());

            let arr2: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
        ];
        let expected2: Vec<Complex<f64>> = vec![
            Complex::new(10.0, 0.0),
            Complex::new(-2.0, 2.0),
            Complex::new(-2.0, 0.0),
            Complex::new(-2.0, -2.0),
        ];
        let fft2 = fft::FFT::new(arr2.len());
        let result2: Vec<Complex<f64>> = fft2.fft(&mut arr2.clone());
        assert_eq!(utils::round_array(result2.clone()), expected2);
        assert_eq!(utils::round_array(fft2.ifft(&mut result2.clone())), arr2.clone());

        let arr3: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
            Complex::new(5.0, 0.0),
            Complex::new(6.0, 0.0),
            Complex::new(7.0, 0.0),
            Complex::new(8.0, 0.0),
        ];
        let expected3: Vec<Complex<f64>> = vec![
            Complex::new(36.0,  0.0),
            Complex::new(-4.0,  9.6568542495),
            Complex::new(-4.0,  4.0),
            Complex::new(-4.0,  1.6568542495),
            Complex::new(-4.0,  0.0),
            Complex::new(-4.0, -1.6568542495),
            Complex::new(-4.0, -4.0),
            Complex::new(-4.0, -9.6568542495),
        ];

        let fft3 = fft::FFT::new(arr3.len());
        let result3: Vec<Complex<f64>> = fft3.fft(&mut arr3.clone());
        assert_eq!(utils::round_array(result3.clone()), expected3);
        assert_eq!(utils::round_array(fft3.ifft(&mut result3.clone())), arr3.clone());
    }

    #[test]
    fn fft_returns_a_symmetrical_array() {
        let n: usize = 64;
        let mut arr: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), n);
        for i in 0 .. n {
            arr[i] = Complex::new(i as f64, 0.0);
        }

        let fft = fft::FFT::new(arr.len());
        let result: Vec<Complex<f64>> = fft.fft(&mut arr);

        let sum: f64 = (n as f64 * (n as f64 - 1.0)) / 2.0;
        let first_element: Complex<f64> = result[0];
        let middle_element: Complex<f64> = result[n / 2];

        assert_eq!(first_element.im, 0.0);
        assert_eq!(first_element.re, sum);
        assert_eq!(middle_element.im, 0.0);
        assert!(utils::fft_array_is_symmetrical(result));
    }

    #[test]
    fn arrays_are_resized_to_power_of_2() {

        let mut arr: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), 1);
        let fft = fft::FFT::new(arr.len());
        let res = fft.fft(&mut arr);
        assert_eq!(res.len(), 1);

        let mut arr2: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), 3);
        let fft2 = fft::FFT::new(arr2.len());
        let res2 = fft2.fft(&mut arr2);
        assert_eq!(res2.len(), 4);

        let mut arr3: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), 120);
        let fft3 = fft::FFT::new(arr3.len());
        let res3 = fft3.fft(&mut arr3);
        assert_eq!(res3.len(), 128);

        let mut arr4: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), 1000);
        let fft4 = fft::FFT::new(arr4.len());
        let res4 = fft4.fft(&mut arr4);
        assert_eq!(res4.len(), 1024);
    }

    #[test]
    fn fft_frequency_bins_are_collected() {
        let sample_rate: f64 = 44100.0;
        let fft = fft::FFT::new(1024);
        let result: Vec<f64> = fft.get_frequency_bins(sample_rate);

        assert_eq!(0.0, result[0]);
        assert_eq!(22006.93359375, result[fft.size / 2 - 1]);

        for i in 1 .. result.len() {
            assert!(result[i-1] < result[i]);
        }
    }

    #[test]
    fn fft_frequency_magnitudes_are_collected() {
        let size = 8;
        let fft = fft::FFT::new(size);
        let arr: Vec<Complex<f64>> = vec![
            Complex::new(36.0,  0.0),
            Complex::new(-4.0,  9.6568542495),
            Complex::new(-4.0,  4.0),
            Complex::new(-4.0,  1.6568542495),
            Complex::new(-4.0,  0.0),
            Complex::new(-4.0, -1.6568542495),
            Complex::new(-4.0, -4.0),
            Complex::new(-4.0, -9.6568542495),
        ];
        let result = fft.get_frequency_magnitudes(arr.clone());

        assert_eq!(size, arr.len());
        assert_eq!(4.5, result[0]);
    }
}
