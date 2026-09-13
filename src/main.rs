use signaalinsuodatin::{io, utils, fft};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arguments = io::parse_cli_arguments(&args);
    match arguments {
        Ok(arguments) => {

            let input_file = arguments.input_file;
            let output_file = arguments.output_file;

            let reader = io::read_input_file(&input_file);
            let spec = reader.spec();
            let samples: Vec<i32> = reader.into_samples()
                                              .map(|r| r.unwrap())
                                              .collect();


            println!("Converting to complex vector.");
            let mut complex_samples = utils::convert_to_complex_vector(samples);

            println!("Padding with zeros.");
            utils::pad_with_zeros(&mut complex_samples);
            println!("Size after padding with zeros {}.", complex_samples.len());

            println!("Performing FFT.");
            let sample_frequencies = fft::fft(complex_samples, false);

            println!("Performing IFFT.");
            let processed_samples = fft::fft(sample_frequencies, true);

            println!("Converting to real vector.");
            let new_samples = utils::convert_to_real_vector(processed_samples);

            // println!("{:?}", complex_vector);
            io::write_output_file(&output_file, spec, new_samples);

        }
        _ => io::print_usage(),
    }
}
