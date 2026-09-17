use crate::{utils, fft, io};
use num::Complex;
use hound;

pub struct Filter {
    output_file: String,
    cutoff_frequency: f64,
    spec: hound::WavSpec,
    samples: Vec<i32>,
}

impl Filter {
    pub fn new() -> Result<Self, &'static str> {

        let arguments: Vec<String> = std::env::args().collect();
        let parsed_arguments = io::parse_cli_arguments(&arguments);

        match parsed_arguments{
            Ok(args) => {

                println!("0/4: Reading file '{}'", args.input_file);
                let reader = io::read_input_file(&args.input_file);
                let spec = reader.spec();
                let samples: Vec<i32> = reader.into_samples()
                    .map(|r| r.unwrap())
                    .collect();

                Ok(Self {
                    output_file: args.output_file,
                    cutoff_frequency: args.cutoff_frequency as f64,
                    spec: spec,
                    samples: samples,
                })
            },

            Err(e) => {
                io::print_usage();
                Err(e)
            },
        }

    }

    pub fn run(&self) {
        let filter = Self::create_low_pass_filter(self.samples.len() + 1, self.spec.sample_rate as f64, self.cutoff_frequency);
        let prepares_samples: Vec<Complex<f64>> = Self::create_prepared_samples(self.samples.clone());
        let processed_samples: Vec<i32> = Self::apply_filter(prepares_samples, filter, self.samples.len());
        Self::write_result_to_file(&self.output_file, self.spec, processed_samples);
    }

    // https://ccrma.stanford.edu/~jos/sasp/Example_1_Low_Pass_Filtering.html
    fn create_low_pass_filter(length: usize, sampling_rate: f64, cutoff_frequency: f64) -> Vec<Complex<f64>> {
        println!("\n1/4: Creating filter.");

        let mut hsupp: Vec<f64> = std::vec::from_elem(0.0, length);
        hsupp[0] = -(length as f64) / 2.0;
        for i in 1 .. length {
            hsupp[i] = hsupp[i - 1] + 1.0;
        }

        let mut hideal: Vec<Complex<f64>> = std::vec::from_elem(Complex::new(0.0, 0.0), length);
        for i in 0.. length {
            let x = (2.0 * cutoff_frequency / sampling_rate) * utils::sinc(2.0 * cutoff_frequency * hsupp[i] / sampling_rate);
            hideal[i] = Complex::new(x, 0.0);
        }

        // https://en.wikipedia.org/wiki/Window_function
        let hamming: Vec<Complex<f64>> = utils::create_hamming_window(length);
        let mut filter: Vec<Complex<f64>> = utils::multiply_complex_arrays(hamming, hideal);

        println!("\t-Padding filter with zeros.");
        utils::pad_with_zeros(&mut filter);

        println!("\t-Performing FFT on filter.");
        let filter = fft::fft(filter, false);

        println!("\t-Filter created.");
        filter
    }

    fn create_prepared_samples(samples: Vec<i32>) -> Vec<Complex<f64>> {
        println!("\n2/4: Preparing samples.");
        println!("\t -Converting samples to complex array.");
        let mut complex_samples = utils::convert_to_complex_array(samples);

        println!("\t -Padding samples with zeros.");
        utils::pad_with_zeros(&mut complex_samples);
        println!("\t -Size after padding with zeros: {}.", complex_samples.len());

        println!("\t -Performing FFT on samples.");
        return fft::fft(complex_samples, false);
    }

    fn apply_filter(prepared_samples: Vec<Complex<f64>>, filter: Vec<Complex<f64>>, old_length: usize) -> Vec<i32> {
        println!("\n3/4: Applying filter on samples.");
        let filtered = utils::multiply_complex_arrays(prepared_samples, filter);
        println!("\t -Filter applied.");

        println!("\t -Performing IFFT on filtered samples.");
        let processed_samples = fft::fft(filtered, true);

        println!("\t -Converting samples back to real array.");
        let mut new_samples = utils::convert_to_real_array(processed_samples);
        new_samples.rotate_left(old_length / 2);
        println!("\t -Resizing samples back to original length.");
        new_samples.resize(old_length, 0);
        new_samples
    }
    fn write_result_to_file(output_file: &str, spec: hound::WavSpec, new_samples: Vec<i32>) -> () {
        println!("\n4/4: Writing output file '{}'", output_file);
        io::write_output_file(output_file, spec, new_samples);
        println!("\t -Output file '{}' created.", output_file);
    }

}

