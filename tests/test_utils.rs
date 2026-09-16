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
            let result = utils::round_c64(numbers[i]);
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
    fn arrays_are_resized_to_power_of_2() {

        let mut arr: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), 1);
        utils::pad_with_zeros(&mut arr);
        assert_eq!(arr.len(), 1);

        let mut arr2: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), 3);
        utils::pad_with_zeros(&mut arr2);
        assert_eq!(arr2.len(), 4);

        let mut arr3: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), 120);
        utils::pad_with_zeros(&mut arr3);
        assert_eq!(arr3.len(), 128);

        let mut arr4: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), 1000);
        utils::pad_with_zeros(&mut arr4);
        assert_eq!(arr4.len(), 1024);

        let mut arr5: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), 500000);
        utils::pad_with_zeros(&mut arr5);
        assert_eq!(arr5.len(), 524288);
    }

    #[test]
    fn conversion_between_real_and_complex_arrays_works() {
        let real: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let complex: Vec<Complex<f64>> = vec![
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
            Complex::new(5.0, 0.0),
            Complex::new(6.0, 0.0),
            Complex::new(7.0, 0.0),
        ];
        assert_eq!(utils::convert_to_complex_array(real.clone()), complex);
        assert_eq!(real, utils::convert_to_real_array(complex));
    }

    #[test]
    fn complex_arrays_are_multiplied_correctly() {
        let arr1: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 1.0),
            Complex::new(2.0, 2.0),
            Complex::new(3.0, 3.0),
            Complex::new(4.0, 4.0),
            Complex::new(5.0, -1.0),
            Complex::new(6.0, -2.0),
            Complex::new(7.0, -3.0),
            Complex::new(8.0, -4.0),
        ];
        let expected: Vec<Complex<f64>> = vec![
            Complex::new(0.0, 2.0),
            Complex::new(0.0, 8.0),
            Complex::new(0.0, 18.0),
            Complex::new(0.0, 32.0),
            Complex::new(24.0, -10.0),
            Complex::new(32.0, -24.0),
            Complex::new(40.0, -42.0),
            Complex::new(48.0, -64.0),
        ];
        assert_eq!(utils::multiply_complex_arrays(arr1.clone(), arr1), expected);
    }

    #[test]
    fn sinc_works_correctly() {
        let arr: Vec<f64> = vec![
            0.0,
            1.0,
            0.1,
            0.2,
            0.3,
            0.4,
            0.5,
            0.6,
        ];
        let expected: Vec<f64> = vec![
            1.0000000000,
            0.0000000000,
            0.9836316431,
            0.9354892838,
            0.8583936913,
            0.7568267286,
            0.6366197724,
            0.5045511524,
        ];
        for i in 0 .. arr.len() {
            assert_eq!(utils::round_f64(utils::sinc(arr[i])), expected[i]);
        }
    }
}
