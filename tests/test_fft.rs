use signaalinsuodatin::fft;
use num::complex::Complex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_are_rounded_correctly() {
        let numbers: Vec<f64> = vec![
            0.80901699437495652,
            2.23933693888498689,
           10.89861345234871623,
          -10.89861345234871623,
           -2.23933693888498689,
            0.80901699437495652,
        ];

        let expected: Vec<f64> = vec![
            0.80901699437496,
            2.23933693888499,
           10.89861345234872,
          -10.89861345234872,
           -2.23933693888499,
            0.80901699437496,
        ];

        for i in 0 .. numbers.len() {
            let result = fft::round(numbers[i]);
            assert_eq!(expected[i], result);
        }
    }

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

        let result = fft::get_even_elements(&arr);

        let expected: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(5.0, 0.0),
            Complex::new(7.0, 0.0),
        ];

        assert_eq!(result, expected);

        let arr2: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
            Complex::new(5.0, 0.0),
        ];

        let result2 = fft::get_even_elements(&arr2);

        let expected2: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 0.0),
            Complex::new(3.0, 0.0),
        ];

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

        let result = fft::get_odd_elements(&arr);

        let expected: Vec<Complex<f64>> = vec![
            Complex::new(2.0, 0.0),
            Complex::new(4.0, 0.0),
            Complex::new(6.0, 0.0),
            Complex::new(8.0, 0.0),
        ];


        assert_eq!(result, expected);

        let arr2: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
            Complex::new(5.0, 0.0),
        ];

        let result2 = fft::get_odd_elements(&arr2);

        let expected2: Vec<Complex<f64>> = vec![
            Complex::new(2.0, 0.0),
            Complex::new(4.0, 0.0),
        ];

        assert_eq!(result2, expected2);
    }

    #[test]
    fn nth_roots_of_unity_are_generated_correctly() -> () {

        let expected_conjugate_roots: Vec<Complex<f64>> = vec![
            Complex::new( 1.0, 0.0),
            Complex::new( 0.80901699437495,  -0.58778525229247),
            Complex::new( 0.30901699437495,  -0.95105651629515),
            Complex::new(-0.30901699437495,  -0.95105651629515),
            Complex::new(-0.80901699437495,  -0.58778525229247),
            Complex::new(-1.0,                0.0),
            Complex::new(-0.80901699437495,   0.58778525229247),
            Complex::new(-0.30901699437495,   0.95105651629515),
            Complex::new( 0.30901699437495,   0.95105651629515),
            Complex::new( 0.80901699437495,   0.58778525229247),
        ];

        let expected_roots: Vec<Complex<f64>> = vec![
            Complex::new( 1.0, 0.0),
            Complex::new( 0.80901699437495,   0.58778525229247),
            Complex::new( 0.30901699437495,   0.95105651629515),
            Complex::new(-0.30901699437495,   0.95105651629515),
            Complex::new(-0.80901699437495,   0.58778525229247),
            Complex::new(-1.0,                0.0),
            Complex::new(-0.80901699437495,  -0.58778525229247),
            Complex::new(-0.30901699437495,  -0.95105651629515),
            Complex::new( 0.30901699437495,  -0.95105651629515),
            Complex::new( 0.80901699437495,  -0.58778525229247),
        ];

        let n = expected_roots.len();

        for k in 0 .. n {
            let nth_root_conjugate: Complex<f64> = fft::gen_nth_root_of_unity(k, n, true);
            let nth_root: Complex<f64> = fft::gen_nth_root_of_unity(k, n, false);

            assert_eq!(nth_root_conjugate, expected_conjugate_roots[k]);
            assert_eq!(nth_root, expected_roots[k]);
        }
    }

    #[test]
    fn fft_is_working() -> () {

        let arr: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
        ];

        let result: Vec<Complex<f64>> = fft::fft(arr, false);

        let expected: Vec<Complex<f64>> = vec![
            Complex::new(3.0, 0.0),
            Complex::new(-1.0, 0.0),
        ];

        assert_eq!(result, expected);

        let arr2: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
        ];

        let result2: Vec<Complex<f64>> = fft::fft(arr2, false);

        let expected2: Vec<Complex<f64>> = vec![
            Complex::new(10.0, 0.0),
            Complex::new(-2.0, 2.0),
            Complex::new(-2.0, 0.0),
            Complex::new(-2.0, -2.0),
        ];

        assert_eq!(result2, expected2);

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

        let result3: Vec<Complex<f64>> = fft::fft(arr3, false);

        let expected3: Vec<Complex<f64>> = vec![
            Complex::new(36.0,  0.0),
            Complex::new(-4.0,  9.656854249492401),
            Complex::new(-4.0,  4.0),
            Complex::new(-4.0,  1.6568542494924001),
            Complex::new(-4.0,  0.0),
            Complex::new(-4.0, -1.6568542494924001),
            Complex::new(-4.0, -4.0),
            Complex::new(-4.0, -9.656854249492401),
        ];

        assert_eq!(result3, expected3);


        let n: usize = 64;
        let mut arr4: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), n);
        for i in 0 .. n {
            arr4[i] = Complex::new(i as f64, 0.0);
        }

        let result4: Vec<Complex<f64>> = fft::fft(arr4, false);

        let first_element: Complex<f64> = result4[0];
        let middle_element: Complex<f64> = result4[n / 2];

        assert_eq!(first_element.im, 0.0);
        assert_eq!(middle_element.im, 0.0);

        for i in 1 .. n {
            let left: Complex<f64> = result4[i];
            let right: Complex<f64> = result4[n - i];
            assert_eq!(left, right.conj());
        }
    }

    #[test]
    fn ifft_is_working() {

        let arr: Vec<Complex<f64>> = vec![
            Complex::new(3.0, 0.0),
            Complex::new(-1.0, 0.0),
        ];

        let expected: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
        ];

        let result: Vec<Complex<f64>> = fft::fft(arr, true);

        assert_eq!(result, expected);

        let arr2: Vec<Complex<f64>> = vec![
            Complex::new(10.0, 0.0),
            Complex::new(-2.0, 2.0),
            Complex::new(-2.0, 0.0),
            Complex::new(-2.0, -2.0),
        ];

        let expected2: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
        ];

        let result2: Vec<Complex<f64>> = fft::fft(arr2, true);

        assert_eq!(result2, expected2);
    }
}
