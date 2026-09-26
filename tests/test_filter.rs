use signaalinsuodatin::{filter, utils};
use num::complex::Complex;

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLES_LENGTH: usize = 100;
    const SAMPLE_RATE: f64 = 48000.0;
    const CUTOFF_FREQUENCY: f64 = 100.0;

    #[test]
    fn coefficients_are_created_correctly() {
        let size: usize = 8;
        let result: Vec<f64> = filter::Filter::create_coefficients(size);
        assert_eq!(result[0], -result[size-1]);

        let size2: usize = 20;
        let result2: Vec<f64> = filter::Filter::create_coefficients(size2);
        assert_eq!(result2[0], -result2[size2-1]);

        let size3: usize = 27;
        let result3: Vec<f64> = filter::Filter::create_coefficients(size3);
        assert_eq!(result3[0], -result3[size3-1]);
    }

    #[test]
    fn ideal_filter_is_calculated_correctly() {
        let coefficients: Vec<f64> = filter::Filter::create_coefficients(SAMPLES_LENGTH);
        let ideal: Vec<Complex<f64>> = filter::Filter::create_ideal(coefficients, CUTOFF_FREQUENCY, SAMPLE_RATE);
        assert_eq!(ideal.len(), SAMPLES_LENGTH);
        assert_eq!(ideal[0], ideal[SAMPLES_LENGTH-1]);
    }

    #[test]
    fn low_pass_filter_is_created() {
        let samples: Vec<i32> = std::vec::from_elem(1000, SAMPLES_LENGTH);
        let filter = filter::Filter::new(CUTOFF_FREQUENCY, SAMPLE_RATE, samples);

        let lpf = filter.lpf;
        assert_eq!(lpf.len(), SAMPLES_LENGTH.next_power_of_two());
        assert_eq!(utils::fft_array_is_symmetrical(lpf), true);
    }

    #[test]
    fn samples_are_prepared_for_filtering() {
        let samples: Vec<i32> = std::vec::from_elem(1000, SAMPLES_LENGTH);
        let filter = filter::Filter::new(CUTOFF_FREQUENCY, SAMPLE_RATE, samples);

        let prepared_samples = filter.create_frequency_samples();
        assert_eq!(prepared_samples.len(), SAMPLES_LENGTH.next_power_of_two());
        assert_eq!(utils::fft_array_is_symmetrical(prepared_samples), true);
    }

    #[test]
    fn filter_is_applied_to_samples() {
        let samples: Vec<i32> = std::vec::from_elem(1000, SAMPLES_LENGTH);
        let samples_sum: i32 = samples.iter().sum();

        let filter = filter::Filter::new(CUTOFF_FREQUENCY, SAMPLE_RATE, samples);
        let prepared_samples = filter.create_frequency_samples();
        let filtered_samples = filter.apply_filter(prepared_samples);
        let filtered_sum: i32 = filtered_samples.iter().sum();

        assert_eq!(filtered_samples.len(), SAMPLES_LENGTH);
        assert_eq!(filtered_samples, filter.get_filtered_samples());
        assert!(filtered_sum < samples_sum);
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
        assert_eq!(signaalinsuodatin::filter::Filter::convert_to_complex_samples(real.clone()), complex);
        assert_eq!(real, signaalinsuodatin::filter::Filter::convert_to_real_samples(complex));
    }

    #[test]
    fn signals_are_convolved_correctly() {
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
        assert_eq!(signaalinsuodatin::filter::Filter::convolve_signals(arr1.clone(), arr1), expected);
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
            assert_eq!(utils::round_float(signaalinsuodatin::filter::Filter::sinc(arr[i])), expected[i]);
        }
    }

    #[test]
    fn hamming_window_is_created_correctly() {
        let n: usize = 10;
        let result: Vec<Complex<f64>> = signaalinsuodatin::filter::Filter::create_hamming_window(n);
        let expected: Vec<Complex<f64>> = vec![
            Complex::new(0.07672,      0.0),
            Complex::new(0.0993142698, 0.0),
            Complex::new(0.1648853947, 0.0),
            Complex::new(0.2670148161, 0.0),
            Complex::new(0.3957053947, 0.0),
            Complex::new(0.53836,      0.0),
            Complex::new(0.6810146053, 0.0),
            Complex::new(0.8097051839, 0.0),
            Complex::new(0.9118346053, 0.0),
            Complex::new(0.9774057302, 0.0),
        ];

        for i in 0 .. n {
            assert_eq!(utils::round_cmplx(result[i]), expected[i]);
        }
    }

}
