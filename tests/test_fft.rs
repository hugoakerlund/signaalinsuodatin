use signaalinsuodatin::fft;
use num::complex::Complex;
use signaalinsuodatin::fft::ROUND_TO_DECIMALS;


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
        let n: usize = 3;

        let result: Vec<Complex<f64>> = fft::gen_nth_roots_of_unity(n, false);

        let expected: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 0.0),
            Complex::new(-0.5, 0.866025),
            Complex::new(-0.5, -0.866025),
        ];

        assert_eq!(result, expected);

        let result2: Vec<Complex<f64>> = fft::gen_nth_roots_of_unity(n, true);

        let expected2: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 0.0),
            Complex::new(-0.5, -0.866025),
            Complex::new(-0.5, 0.866025),
        ];

        assert_eq!(result2, expected2);
    }
}
