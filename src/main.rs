use signaalinsuodatin::io;

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
            io::write_output_file(&output_file, spec, samples);

        }
        _ => io::print_usage(),
    }
}
