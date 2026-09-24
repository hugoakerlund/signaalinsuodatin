use signaalinsuodatin::{filter, io};

fn main() {

    // Argumentit luetaan komentoriviltä ja jäsennetään.
    let arguments: Vec<String> = std::env::args().collect();
    let parsed_arguments = io::parse_cli_arguments(&arguments);

    // Jäsennetyt argumentit käsitellään.
    match parsed_arguments {

        // Jos argumentit ovat hyväksytyt, suoritusta jatketaan.
        Ok(args) => {

            // Äänitiedosto luetaan levyltä.
            let file = io::read_input_file(&args.input_file);
            let spec = file.spec();
            let sample_rate = spec.sample_rate as f64;
            let samples: Vec<i32> = file.into_samples()
                .map(|r| r.unwrap())
                .collect();

            // Suodatin luodaan luetun tiedoston ja annetun ylärajataajuuden perusteella.
            let filter = filter::Filter::new(args.cutoff_frequency, sample_rate, samples);

            // Ääninäytteet suodatetaan.
            let filtered_samples = filter.get_filtered_samples();

            // Suodatetut ääninäytteet kirjoitetaan levylle.
            io::write_output_file(&args.output_file, spec, filtered_samples);
        },

        // Jos argumentit ovat virheelliset, suoritus lopetataan ja käyttöohje tulostetaan.
        _ => {
            io::print_usage();
        }
    }
}
