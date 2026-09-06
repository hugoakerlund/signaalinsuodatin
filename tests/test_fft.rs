use signaalinsuodatin::fft;
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

        let result = fft::get_even_elements(&arr);

        let expected: Vec<Complex<f64>> = vec![
            Complex::new(2.0, 0.0),
            Complex::new(4.0, 0.0),
            Complex::new(6.0, 0.0),
            Complex::new(8.0, 0.0),
        ];

        assert_eq!(result, expected);
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
            Complex::new(1.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(5.0, 0.0),
            Complex::new(7.0, 0.0),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn nth_roots_of_unity_are_generated_correctly() -> () {
        let n: usize = 3;

        let result: Vec<Complex<f64>> = fft::get_nth_roots_of_unity(n);

        let expected: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 0.0),
            Complex::new(-0.5, 0.866025),
            Complex::new(-0.5, -0.866025),
        ];

        assert_eq!(result, expected);

    }
}
