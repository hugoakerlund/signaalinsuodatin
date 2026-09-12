use std::fs::File;
use std::io::BufReader;
use hound;

// Tietue sisältää komentoriviltä ohjelmalle annetut argumentit.
pub struct Arguments {
    pub input_file: String,
    pub output_file: String,
    pub cutoff_frequency: u16,
}

// Funktio lukee syöte tiedoston levyltä ja tulostaa tietoja sen spesifikaatiosta. Tiedoston lukija
// palautetaan.
pub fn read_input_file(input_file: &str) -> hound::WavReader<BufReader<File>> {
    println!("Reading file '{}'\n", input_file);
    let mut reader = hound::WavReader::open(input_file).unwrap();
    let spec = reader.spec();

    let channels = spec.channels;
    let sample_rate = spec.sample_rate;
    let bits_per_sample = spec.bits_per_sample;
    let duration = reader.duration() / sample_rate;
    let samples = reader.samples::<i32>().len();

    println!("File information:");
    println!("\tChannels: {:?}", channels);
    println!("\tSamples rate: {:?}", sample_rate);
    println!("\tBits per sample: {:?}", bits_per_sample);
    println!("\tDuration: {}min {}s", duration / 60, duration % 60);
    println!("\tNumber of samples: {:?}\n", samples);

    reader
}

// Funktio kirjoittaa tiedoston levylle. Funktio saa argumentteina tiedoston nimen, spesifikaation ja
// ääninäytteet.
pub fn write_output_file(output_file: &str, spec: hound::WavSpec, samples: Vec<i32>) -> () {
    println!("Writing file '{}'", output_file);
    let mut writer = hound::WavWriter::create(output_file, spec).unwrap();
    for sample in samples {
        writer.write_sample(sample).unwrap();
    }
}

// Funktio jäsentää komentoriviltä annetut argumentit ja palauttaa tietueen, jos argumentit ovat
// oikeanlaiset.
pub fn parse_cli_arguments(args: &Vec<String>) -> Result<Arguments, &'static str> {

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
            let parsed = val.parse::<u16>();
            match parsed {
                Ok(parsed) => {cutoff = parsed}
                _ => return Err("Cutoff frequency must be a positive integer.")

            }
        }
        else {
            return Err("Invalid argument.")
        }
    }

    Ok(Arguments { input_file: input, output_file: output, cutoff_frequency: cutoff})
}

// Funktio tulostaa lyhyesti ohjelman käyttötavan.
pub fn print_usage() -> () {
    println!("./signaalinsuodatin --input <input_file --output <output_file> --cutoff <cutoff_frequency>");
}
