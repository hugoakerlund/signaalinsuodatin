use signaalinsuodatin::{io, utils, fft};
use num::Complex;

// https://ccrma.stanford.edu/~jos/sasp/Example_1_Low_Pass_Filtering.html
fn create_low_pass_filter(length: usize, sampling_rate: f64, cutoff_frequency: f64) -> Vec<Complex<f64>> {
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
    let hamming: Vec<Complex<f64>> = utils::hamming(length);
    let mut filter: Vec<Complex<f64>> = utils::multiply_complex_arrays(hamming, hideal);
    println!("Padding filter with zeros.");
    utils::pad_with_zeros(&mut filter);
    filter
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arguments = io::parse_cli_arguments(&args);
    match arguments {
        Ok(arguments) => {

            let input_file = arguments.input_file;
            let output_file = arguments.output_file;
            let cutoff = arguments.cutoff_frequency;

            let reader = io::read_input_file(&input_file);
            let spec = reader.spec();
            let samples: Vec<i32> = reader.into_samples()
                                              .map(|r| r.unwrap())
                                              .collect();

            let sampling_rate = spec.sample_rate;
            let filter_length = samples.len() + 1;
            let old_length = filter_length;

            println!("Creating low-pass filter.");
            let filter: Vec<Complex<f64>> = create_low_pass_filter(filter_length, sampling_rate as f64, cutoff as f64);
            // println!("{:?}", filter);

            println!("Performing FFT on filter.");
            let fft_filter = fft::fft(filter, false);

            println!("Converting samples to complex array.");
            let mut complex_samples = utils::convert_to_complex_array(samples);

            println!("Padding samples with zeros.");
            utils::pad_with_zeros(&mut complex_samples);
            let padded_length = complex_samples.len();
            println!("Size after padding with zeros {}.", padded_length);

            println!("Performing FFT on samples.");
            let frequencies = fft::fft(complex_samples, false);

            println!("Applying filter on samples.");
            let filtered = utils::multiply_complex_arrays(frequencies, fft_filter);
            // println!("{:?}", filtered);

            println!("Performing IFFT on samples.");
            let processed_samples = fft::fft(filtered, true);

            println!("Converting samples back to real array.");
            let mut new_samples = utils::convert_to_real_array(processed_samples);
            new_samples.rotate_left(old_length / 2);
            new_samples.resize(old_length, 0);

            io::write_output_file(&output_file, spec, new_samples);

        }
        _ => io::print_usage(),
    }
}
