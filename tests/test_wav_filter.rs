use signaalinsuodatin::{wav_filter, utils};

#[cfg(test)]
mod tests {
    use super::*;

    const CUTOFF_FREQUENCY: f64 = 800.0;
    const PERCENTAGE_OVER_CUTOFF: f64 = 3.0;

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
    fn test_samples_are_filtered_correctly() {

        let files: Vec<&str> = vec![
            "test_data/sample-3s-mono.wav",
            "test_data/sample-3s-stereo.wav",
            "test_data/sample-3s.wav",
            // "test_data/sample-6s.wav",
            // "test_data/sample-9s.wav",
            // "test_data/sample-12s.wav",
            // "test_data/sample-19s.wav",
        ];

        for file in files {
            assert!(utils::get_filtered_percentage_over_cutoff(file, CUTOFF_FREQUENCY) < PERCENTAGE_OVER_CUTOFF);
        }
    }

    #[test]
    fn synthetic_files_are_filtered_correctly() {

        let files: Vec<&str> = vec![
            "test_data/synthetic_samples/100Hz-1000Hz.wav",
            "test_data/synthetic_samples/440Hz-5000Hz.wav",
            "test_data/synthetic_samples/400Hz-500Hz.wav",
            "test_data/synthetic_samples/10000Hz-20000Hz.wav",
        ];

        for file in files {
            assert!(utils::get_filtered_percentage_over_cutoff(file, CUTOFF_FREQUENCY) < PERCENTAGE_OVER_CUTOFF);
        }
    }

    #[test]
    fn recordings_are_filtered_correctly() {

        let files: Vec<&str> = vec![
            "test_data/recorded_samples/bourree.wav",
            "test_data/recorded_samples/guajiras.wav",
            // "test_data/recorded_samples/asturias.wav",
            // "test_data/recorded_samples/recuerdos.wav",
            // "test_data/recorded_samples/chaconne.wav",
            // "test_data/recorded_samples/caprice.wav",
        ];

        for file in files {
            assert!(utils::get_filtered_percentage_over_cutoff(file, CUTOFF_FREQUENCY) < PERCENTAGE_OVER_CUTOFF);
        }
    }
}
