use std::fs::File;
use std::io::BufReader;
use std::env;
use hound;

struct Arguments {
    input_file: String,
    output_file: String,
    cutoff_frequency: u16,
}

fn read_input_file(input_file: &str) -> hound::WavReader<BufReader<File>> {
    let mut reader = hound::WavReader::open(input_file).unwrap();
    let spec = reader.spec();

    let channels = spec.channels;
    let sample_rate = spec.sample_rate;
    let bits_per_sample = spec.bits_per_sample;
    let duration = reader.duration() / sample_rate;
    let samples = reader.samples::<i32>().len();

    println!("Channels: {:?}", channels);
    println!("Samples rate: {:?}", sample_rate);
    println!("Bits per sample: {:?}", bits_per_sample);
    println!("Duration: {}min {}s", duration / 60, duration % 60);
    println!("Samples: {:?}", samples);

    reader

}

fn write_output_file(output_file: &str, mut reader: hound::WavReader<BufReader<File>>) -> () {
    let spec = reader.spec();
    let mut writer = hound::WavWriter::create(output_file, spec).unwrap();
    for sample in reader.samples::<i32>() {
        let sample = sample.unwrap();
        writer.write_sample(sample).unwrap();
    }
}

fn parse_cli_arguments(args: &Vec<String>) -> Result<Arguments, &'static str> {

    if args.len() != 7 {
        return Err("Invalid argument count.");
    }

    let mut input: String = String::new();
    let mut output: String = String::new();
    let mut cutoff: u16 = 0;

    for i in (1..args.len()).step_by(2) {

        let opt = &args[i];
        let val = &args[i + 1];

        if opt == "--input" {
            input = val.to_string();
        }
        else if opt == "--output" {
            output = val.to_string()
        }
        else if opt == "--cutoff" {
            cutoff = val.parse::<u16>().unwrap();
        }
        else {
            print_usage();
            return Err("Invalid argument.")
        }
    }

    Ok(Arguments { input_file: input, output_file: output, cutoff_frequency: cutoff})

}

fn print_usage() -> () {
    println!("./signaalinsuodatin --input <input_file --output <output_file> --cutoff <cutoff_frequency>");
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = parse_cli_arguments(&args).unwrap();

    let input_file = arguments.input_file;
    let output_file = arguments.output_file;

    let reader = read_input_file(&input_file);
    write_output_file(&output_file, reader);
}
