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
            let result = utils::round(numbers[i]);
            assert_eq!(expected[i], result);
        }
    }

    #[test]
    pub fn arrays_are_rounded_correctly() {
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
}
