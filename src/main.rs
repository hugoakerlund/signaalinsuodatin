use signaalinsuodatin::{io, wav_filter};


fn main() {

    // Luodaan IO-operaatioita tekevä olio.
    let io = io::IO::new();

    // Argumentit luetaan komentoriviltä ja jäsennetään.
    let arguments: Vec<String> = std::env::args().collect();
    let parsed_arguments = io.parse_cli_arguments(&arguments);

    // Jäsennetyt argumentit käsitellään.
    match parsed_arguments {

        // Jos argumentit ovat hyväksytyt, suoritusta jatketaan.
        Ok(args) => {

            // Äänitiedosto luetaan levyltä.
            let file = io.read_input_file(&args.input_file);
            let spec = file.spec();

            // Luodaan äänitiedoston suodatin.
            let wav_filter = wav_filter::WAVFilter::new(file, args.cutoff_frequency);

            // Suodatetaan äänitiedosto.
            let filtered_samples = wav_filter.get_filtered_samples();

            // Suodatetut ääninäytteet kirjoitetaan levylle.
            io.write_output_file(&args.output_file, spec, filtered_samples);

        },

        // Jos argumentit ovat virheelliset, suoritus lopetataan ja käyttöohje tulostetaan.
        _ => {
            io.print_usage();
        }
    }
}
