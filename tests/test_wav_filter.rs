use signaalinsuodatin::{wav_filter, utils, io};

#[cfg(test)]
mod tests {
    use super::*;

    const CUTOFF_FREQUENCY: f64 = 800.0;

    #[test]
    fn left_and_right_channels_are_extracted() {
        let arr: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
        let right_expected: Vec<i32> = vec![2,4,6,8,10];
        let left_expected: Vec<i32> = vec![1,3,5,7,9];
        let (left, right): (Vec<i32>, Vec<i32>) = wav_filter::WAVFilter::extract_left_and_right_channels(arr);

        assert_eq!(right, right_expected);
        assert_eq!(left, left_expected);
    }

    #[test]
    fn left_and_right_channels_are_joined() {
        let left: Vec<i32> = vec![1,3,5,7,9];
        let right: Vec<i32> = vec![2,4,6,8,10];
        let expected: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
        let joined: Vec<i32> = wav_filter::WAVFilter::join_channels(left, right);

        assert_eq!(expected, joined);
    }

    #[test]
    fn higher_frequencies_than_cutoff_are_attenuated() {

        let test_files: Vec<&str> = vec![
            "test_data/sample-3s.wav",
            "test_data/sample-3s-mono.wav",
            // "test_data/sample-6s.wav",
            // "test_data/sample-9s.wav",
            // "test_data/sample-12s.wav",
            // "test_data/sample-19s.wav",
        ];

        for file in test_files {

            let file_to_read = io::read_input_file(file);
            let file_to_filter = io::read_input_file(file);

            let spec = file_to_read.spec();
            let channels = spec.channels;
            let sample_rate = spec.sample_rate as f64;
            let samples: Vec<i32> = file_to_read.into_samples()
                .map(|r| r.unwrap())
                .collect();

            let mut samples_to_read: Vec<i32> = samples.clone();
            if channels == 2 {
                let (left, _right): (Vec<i32>, Vec<i32>) = wav_filter::WAVFilter::extract_left_and_right_channels(samples.clone());
                samples_to_read = left;
            }

            let magnitudes_before: (f64, f64) = utils::get_magnitudes(samples_to_read.clone(), CUTOFF_FREQUENCY, sample_rate);
            let total_magnitude_before = magnitudes_before.0 + magnitudes_before.1;
            let percentage_over_cutoff_before = (magnitudes_before.1 / (total_magnitude_before)) * 100.0;

            println!("total magnitude before filtering {}", total_magnitude_before);
            println!("percentage over cutoff before filtering {} %", percentage_over_cutoff_before);




            let wav_filter = wav_filter::WAVFilter::new(file_to_filter, CUTOFF_FREQUENCY);
            let mut filtered_samples = wav_filter.get_filtered_file();

            if channels == 2 {
                let (left_filtered, _right_filtered): (Vec<i32>, Vec<i32>) = wav_filter::WAVFilter::extract_left_and_right_channels(filtered_samples.clone());
                filtered_samples = left_filtered;
            }

            let magnitudes_after: (f64, f64) = utils::get_magnitudes(filtered_samples.clone(), CUTOFF_FREQUENCY, sample_rate);
            let total_magnitude_after = magnitudes_after.0 + magnitudes_after.1;
            let percentage_over_cutoff_after = (magnitudes_after.1 / (total_magnitude_after)) * 100.0;

            println!("total magnitude after filtering {}", total_magnitude_after);
            println!("percentage over cutoff after filtering {} %", percentage_over_cutoff_after);

            assert!(percentage_over_cutoff_after < percentage_over_cutoff_before);
            assert!(percentage_over_cutoff_after < 2.0);
        }
    }
}
