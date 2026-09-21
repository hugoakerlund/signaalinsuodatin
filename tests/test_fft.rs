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
            Complex::new( 1.0, 0.0),
            Complex::new( 0.8090169944,  0.5877852523),
            Complex::new( 0.3090169944,  0.9510565163),
            Complex::new(-0.3090169944,  0.9510565163),
            Complex::new(-0.8090169944,  0.5877852523),
            Complex::new(-1.0,           0.0),
            Complex::new(-0.8090169944, -0.5877852523),
            Complex::new(-0.3090169944, -0.9510565163),
            Complex::new( 0.3090169944, -0.9510565163),
            Complex::new( 0.8090169944, -0.5877852523),
        ];
        let expected_conjugate_roots: Vec<Complex<f64>> = vec![
            Complex::new( 1.0, 0.0),
            Complex::new( 0.8090169944, -0.5877852523),
            Complex::new( 0.3090169944, -0.9510565163),
            Complex::new(-0.3090169944, -0.9510565163),
            Complex::new(-0.8090169944, -0.5877852523),
            Complex::new(-1.0,           0.0),
            Complex::new(-0.8090169944,  0.5877852523),
            Complex::new(-0.3090169944,  0.9510565163),
            Complex::new( 0.3090169944,  0.9510565163),
            Complex::new( 0.8090169944,  0.5877852523),
        ];

        let n = expected_roots.len();
        let mut nth_roots: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), n);
        let mut nth_roots_conjugate: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), n);

        for i in 0 .. n {
            nth_roots[i] = fft.gen_nth_root_of_unity(i, n, false);
            nth_roots_conjugate[i] = fft.gen_nth_root_of_unity(i, n, true);
        }

        assert_eq!(utils::round_array(nth_roots_conjugate), expected_conjugate_roots);
        assert_eq!(utils::round_array(nth_roots), expected_roots);
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
        let result: Vec<Complex<f64>> = fft.fft(&mut arr.clone(), false);
        assert_eq!(utils::round_array(result.clone()), expected);
        assert_eq!(utils::round_array(fft.fft(&mut result.clone(), true)), arr.clone());

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
        let result2: Vec<Complex<f64>> = fft2.fft(&mut arr2.clone(), false);
        assert_eq!(utils::round_array(result2.clone()), expected2);
        assert_eq!(utils::round_array(fft2.fft(&mut result2.clone(), true)), arr2.clone());

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
        let result3: Vec<Complex<f64>> = fft3.fft(&mut arr3.clone(), false);
        assert_eq!(utils::round_array(result3.clone()), expected3);
        assert_eq!(utils::round_array(fft3.fft(&mut result3.clone(), true)), arr3.clone());
    }

    #[test]
    fn fft_returns_a_symmetrical_array() {
        let n: usize = 64;
        let mut arr: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), n);
        for i in 0 .. n {
            arr[i] = Complex::new(i as f64, 0.0);
        }

        let fft = fft::FFT::new(arr.len());
        let result: Vec<Complex<f64>> = fft.fft(&mut arr, false);

        let first_element: Complex<f64> = result[0];
        let middle_element: Complex<f64> = result[n / 2];

        assert_eq!(first_element.im, 0.0);
        assert_eq!(middle_element.im, 0.0);
        assert_eq!(utils::fft_array_is_symmetrical(result), true);()
    }

    #[test]
    fn arrays_are_resized_to_power_of_2() {

        let mut arr: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), 1);
        let fft = fft::FFT::new(arr.len());
        let res = fft.fft(&mut arr, false);
        assert_eq!(res.len(), 1);

        let mut arr2: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), 3);
        let fft2 = fft::FFT::new(arr2.len());
        let res2 = fft2.fft(&mut arr2, false);
        assert_eq!(res2.len(), 4);

        let mut arr3: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), 120);
        let fft3 = fft::FFT::new(arr3.len());
        let res3 = fft3.fft(&mut arr3, false);
        assert_eq!(res3.len(), 128);

        let mut arr4: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), 1000);
        let fft4 = fft::FFT::new(arr4.len());
        let res4 = fft4.fft(&mut arr4, false);
        assert_eq!(res4.len(), 1024);

        let mut arr5: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), 500000);
        let fft5 = fft::FFT::new(arr5.len());
        let res5 = fft5.fft(&mut arr5, false);
        assert_eq!(res5.len(), 524288);
    }

}
