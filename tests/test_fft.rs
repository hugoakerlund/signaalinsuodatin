use signaalinsuodatin::{fft, utils};
use num::complex::Complex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn even_elements_are_collected() -> () {
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
        let result = fft::get_even_elements(&arr);
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
        ];
        let result2 = fft::get_even_elements(&arr2);
        assert_eq!(result2, expected2);
    }

    #[test]
    fn odd_elements_are_collected() -> () {
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
        let result = fft::get_odd_elements(&arr);
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
        let result2 = fft::get_odd_elements(&arr2);
        assert_eq!(result2, expected2);
    }

    #[test]
    fn nth_roots_of_unity_are_generated_correctly() -> () {
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
            nth_roots[i] = fft::gen_nth_root_of_unity(i, n, false);
            nth_roots_conjugate[i] = fft::gen_nth_root_of_unity(i, n, true);
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
        let result: Vec<Complex<f64>> = fft::fft(arr.clone(), false);
        assert_eq!(utils::round_array(result.clone()), expected);
        assert_eq!(utils::round_array(fft::fft(result.clone(), true)), arr.clone());

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
        let result2: Vec<Complex<f64>> = fft::fft(arr2.clone(), false);
        assert_eq!(utils::round_array(result2.clone()), expected2);
        assert_eq!(utils::round_array(fft::fft(result2.clone(), true)), arr2.clone());

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

        let result3: Vec<Complex<f64>> = fft::fft(arr3.clone(), false);
        assert_eq!(utils::round_array(result3.clone()), expected3);
        assert_eq!(utils::round_array(fft::fft(result3.clone(), true)), arr3.clone());
    }

    #[test]
    fn fft_returns_a_symmetrical_array() {
        let n: usize = 64;
        let mut arr: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), n);
        for i in 0 .. n {
            arr[i] = Complex::new(i as f64, 0.0);
        }

        let result: Vec<Complex<f64>> = fft::fft(arr, false);

        let first_element: Complex<f64> = result[0];
        let middle_element: Complex<f64> = result[n / 2];

        assert_eq!(first_element.im, 0.0);
        assert_eq!(middle_element.im, 0.0);

        for i in 1 .. n {
            let left: Complex<f64> = result[i];
            let right: Complex<f64> = result[n - i];
            assert_eq!(utils::round(left), utils::round(right.conj()));
        }
    }
}
