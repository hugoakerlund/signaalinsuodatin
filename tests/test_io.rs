use signaalinsuodatin::io;
use std::fs::remove_file;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_file_is_read() {
        let input_file: &str = "test_data/input.wav";
        let reader = io::read_input_file(input_file);
        assert_eq!(reader.spec().channels, 2);
        assert_eq!(reader.spec().sample_rate, 48000);
        assert_eq!(reader.spec().bits_per_sample, 16);
    }

    #[test]
    fn output_file_is_written() {
        let output_file: &str = "./output.wav";
        if Path::new(output_file).exists() {
            remove_file(output_file).unwrap();
        }

        let spec = hound::WavSpec {
            bits_per_sample: 16,
            channels: 1,
            sample_format: hound::SampleFormat::Int,
            sample_rate: 16000,
        };

        let num_samples = 100000;
        let mut samples: Vec<i32> = std::vec::from_elem(0, num_samples);

        for i in 0 .. num_samples {
            samples[i] = (i as u16  % spec.bits_per_sample * 1000) as i32;
        }

        io::write_output_file(output_file, spec, samples);
        assert!(Path::new(output_file).exists());
    }

    #[test]
    fn cli_arguments_are_parsed() {
        let args: Vec<String> = vec![
            "./signaalinsuodatin".to_string(),
            "--input".to_string(),
            "input.wav".to_string(),
            "--output".to_string(),
            "output.wav".to_string(),
            "--cutoff".to_string(),
            "100".to_string()
        ];
        let result = io::parse_cli_arguments(&args);
        assert!(result.is_ok());

        let args2: Vec<String> = vec![
            "./signaalinsuodatin".to_string(),
            "--input".to_string(),
            "--output".to_string(),
            "output.wav".to_string(),
            "--cutoff".to_string(),
            "100".to_string()
        ];
        let result2 = io::parse_cli_arguments(&args2);
        assert!(result2.is_err());

        let args3: Vec<String> = vec![
            "./signaalinsuodatin".to_string(),
            "--input".to_string(),
            "input.wav".to_string(),
            "--output".to_string(),
            "output.wav".to_string(),
            "--cutoff".to_string(),
            "-100".to_string()
        ];
        let result3 = io::parse_cli_arguments(&args3);
        assert!(result3.is_err());

        let args4: Vec<String> = vec![
            "./signaalinsuodatin".to_string(),
            "-i".to_string(),
            "input.wav".to_string(),
            "--output".to_string(),
            "output.wav".to_string(),
            "--cutoff".to_string(),
            "100".to_string()
        ];
        let result4 = io::parse_cli_arguments(&args4);
        assert!(result4.is_err());
    }
}
