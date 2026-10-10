use std::fs::File;
use std::io::BufReader;
use hound;

/// Tietue edustaa komentoriviltä ohjelmalle annettuja argumentteja.
pub struct Arguments {
    pub input_file: String,
    pub output_file: String,
    pub cutoff_frequency: f64,
}

/// Tietue edustaa IO-operaatioita tekevää oliota.
pub struct IO {}

impl IO {

    /// Konstruktori luo uuden IO-olion.
    pub fn new() -> Self {
        Self {}
    }

    /// Metodi lukee syöte tiedoston levyltä ja tulostaa tietoja sen spesifikaatiosta. Tiedoston lukija
    /// palautetaan.
    ///
    /// # Esimerkit
    /// ```
    /// let io = signaalinsuodatin::io::IO::new();
    /// let file = io.read_input_file("test_data/sample-3s.wav");
    /// assert_eq!(file.spec().channels, 2);
    /// ```
    pub fn read_input_file(&self, input_file: &str) -> hound::WavReader<BufReader<File>> {
        println!("\n* Reading file: '{}'", input_file);

        let mut reader = hound::WavReader::open(input_file).unwrap();
        let spec = reader.spec();

        let channels = spec.channels;
        let sample_rate = spec.sample_rate;
        let bits_per_sample = spec.bits_per_sample;
        let duration = reader.duration() / sample_rate;
        let samples = reader.samples::<i32>().len();

        println!("\tFile information:");
        println!("\t-Channels: {:?}", channels);
        println!("\t-Sampling rate: {:?}", sample_rate);
        println!("\t-Bits per sample: {:?}", bits_per_sample);
        println!("\t-Duration: {}min {}s", duration / 60, duration % 60);
        println!("\t-Number of samples: {:?}", samples);

        reader
    }

    /// Metodi kirjoittaa tiedoston levylle. Metodi saa argumentteina tiedoston nimen, spesifikaation ja
    /// ääninäytteet.
    ///
    /// # Esimerkit
    /// ```
    /// use std::path::Path;
    /// use std::fs::remove_file;
    ///
    /// let samples: Vec<i32> = std::vec::from_elem(1000, 10000);
    ///
    /// let spec = hound::WavSpec {
    ///     channels: 1,
    ///     sample_rate: 44100,
    ///     bits_per_sample: 16,
    ///     sample_format: hound::SampleFormat::Int,
    /// };
    ///
    /// let io = signaalinsuodatin::io::IO::new();
    /// io.write_output_file("output.wav", spec, samples);
    /// assert!(Path::new("output.wav").exists());
    /// remove_file("output.wav").unwrap();
    /// ```
    pub fn write_output_file(&self, output_file: &str, spec: hound::WavSpec, samples: Vec<i32>) -> () {
        println!("* Writing output file '{}'", output_file);

        let mut writer = hound::WavWriter::create(output_file, spec).unwrap();

        for sample in samples {
            writer.write_sample(sample).unwrap();
        }
    }

    /// Metodi jäsentää komentoriviltä annetut argumentit ja palauttaa tietueen, jos argumentit ovat
    /// oikeanlaiset.
    ///
    /// # Esimerkit
    ///
    /// ```
    /// let args: Vec<String> = vec![
    /// "./signaalinsuodatin".to_string(),
    /// "--input".to_string(), "input.wav".to_string(),
    /// "--output".to_string(), "output.wav".to_string(),
    /// "--cutoff".to_string(), "1000".to_string()];
    ///
    /// let io = signaalinsuodatin::io::IO::new();
    /// let parsed_args = io.parse_cli_arguments(&args).unwrap();
    /// assert_eq!(parsed_args.cutoff_frequency, 1000.0);
    /// ```
    pub fn parse_cli_arguments(&self, args: &Vec<String>) -> Result<Arguments, &'static str> {

        if args.len() != 7 {
            return Err("Invalid argument count.");
        }

        let mut input: String = String::new();
        let mut output: String = String::new();
        let mut cutoff: f64 = 0.0;

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
                    Ok(parsed) => {cutoff = parsed as f64}
                    _ => return Err("Cutoff frequency must be a positive integer.")

                }
            }
            else {
                return Err("Invalid argument.")
            }
        }

        Ok(Arguments { input_file: input, output_file: output, cutoff_frequency: cutoff})
    }

    /// Metodi tulostaa lyhyesti ohjelman käyttöohjeen.
    pub fn print_usage(&self) -> () {
        println!("./signaalinsuodatin --input <input_file> --output <output_file> --cutoff <cutoff_frequency>");
    }
}
