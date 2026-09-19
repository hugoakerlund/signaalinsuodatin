use signaalinsuodatin::{filter, io};

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let parsed_arguments = io::parse_cli_arguments(&arguments);

    match parsed_arguments {
        Ok(args) => {

            let file = io::read_input_file(&args.input_file);
            let spec = file.spec();
            let sample_rate = spec.sample_rate as f64;
            let samples: Vec<i32> = file.into_samples()
                .map(|r| r.unwrap())
                .collect();

            let filter = filter::Filter::new(args.cutoff_frequency, sample_rate, samples);
            let filtered_samples = filter.get_filtered_samples();

            io::write_output_file(&args.output_file, spec, filtered_samples);
        },

        _ => {
            io::print_usage();
        }
    }
}
