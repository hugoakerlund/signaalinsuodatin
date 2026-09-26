use signaalinsuodatin::utils;
use num::complex::Complex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_are_rounded_correctly() {
        let numbers: Vec<Complex<f64>> = vec![
          Complex::new(  0.80901699437495652, 0.0),
          Complex::new(  2.23933693888498689, 0.0),
          Complex::new( 10.89861345234871623, 0.0),
          Complex::new(-10.89861345234871623, 0.0),
          Complex::new( -2.23933693888498689, 0.0),
          Complex::new(  0.80901699437495652, 0.0),
        ];
        let expected: Vec<Complex<f64>> = vec![
          Complex::new(  0.8090169944, 0.0),
          Complex::new(  2.2393369389, 0.0),
          Complex::new( 10.8986134523, 0.0),
          Complex::new(-10.8986134523, 0.0),
          Complex::new( -2.2393369389, 0.0),
          Complex::new(  0.8090169944, 0.0),
        ];
        for i in 0 .. numbers.len() {
            let result = utils::round_cmplx(numbers[i]);
            assert_eq!(expected[i], result);
        }
    }

    #[test]
    pub fn complex_arrays_are_rounded_correctly() {
        let numbers: Vec<Complex<f64>> = vec![
          Complex::new(  0.80901699437495652, 0.0),
          Complex::new(  2.23933693888498689, 0.0),
          Complex::new( 10.89861345234871623, 0.0),
          Complex::new(-10.89861345234871623, 0.0),
          Complex::new( -2.23933693888498689, 0.0),
          Complex::new(  0.80901699437495652, 0.0),
        ];
        let expected: Vec<Complex<f64>> = vec![
          Complex::new(  0.8090169944, 0.0),
          Complex::new(  2.2393369389, 0.0),
          Complex::new( 10.8986134523, 0.0),
          Complex::new(-10.8986134523, 0.0),
          Complex::new( -2.2393369389, 0.0),
          Complex::new(  0.8090169944, 0.0),
        ];
        assert_eq!(utils::round_array(numbers), expected);
    }

    #[test]
    fn fft_arrays_are_tested_for_symmetry() {
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

        let arr2: Vec<Complex<f64>> = vec![
            Complex::new(36.0,  0.0),
            Complex::new(-4.0,  9.6568542495),
            Complex::new(-4.0,  4.0),
            Complex::new(-4.0,  1.6568542495),
            Complex::new(2.0,   0.0),
            Complex::new(2.0,   0.0),
            Complex::new(2.0,   0.0),
            Complex::new(2.0,   0.0),
        ];

        assert_eq!(utils::fft_array_is_symmetrical(arr), true);
        assert_eq!(utils::fft_array_is_symmetrical(arr2), false);
    }
}
