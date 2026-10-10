use std::fs::File;
use std::io::BufReader;
use crate::filter;
use hound;

/// Tietue edustaa WAV tiedoston suodatinta.
pub struct WAVFilter {

    /// Äänitiedoston kanavat.
    channels: u16,

    /// Äänitiedoston ääninäytteet.
    samples: Vec<i32>,

    /// Ylärajataajuus hertseinä.
    cutoff_frequency: f64,

    /// Näytteenottotaajuus hertseinä.
    sample_rate: f64,
}

impl WAVFilter {

    /// Konstruktori luo uuden WAV tiedoston suodattimen ja saa argumentteina tiedoston lukijan ja
    /// ylärajataajuuden.
    pub fn new(file: hound::WavReader<BufReader<File>>, cutoff_frequency: f64) -> Self {
        let spec = file.spec();
        let sample_rate = spec.sample_rate as f64;
        let channels = spec.channels;
        let samples: Vec<i32> = file.into_samples()
            .map(|r| r.unwrap())
            .collect();

        Self {
            channels: channels,
            samples: samples,
            sample_rate: sample_rate,
            cutoff_frequency: cutoff_frequency,
        }

    }

    /// Metodi suodattaa äänitiedoston ja palauttaa suodatetut ääninäytteet. Mikäli kanavia on
    /// kaksi ne eristetään toisistaan, suodatetaan erikseen ja lopuksi yhdistetään.
    pub fn get_filtered_samples(&self) -> Vec<i32> {
        if self.channels == 2 {
            return self.filter_stereo();
        }
        else if self.channels == 1 {
            return self.filter_mono();

        }
        else {
            panic!("Input file has incorrect number of channels.");
        }
    }

    /// Metodi suodattaa kaksikanavaisen äänitiedoston ja palauttaa suodatetut ääninäytteet.
    fn filter_stereo(&self) -> Vec<i32> {

        // Vasen ja oikea kavana erotellaan.
        println!("\n* Extracting left and right channels for filtering.");
        let (left, right) = Self::extract_left_and_right_channels(self.samples.clone());

        // Vasemmalle kanavalle luodaan oma suodatin.
        println!("\n* Creating filter for left channel:");
        let left_filter = filter::Filter::new(self.cutoff_frequency, self.sample_rate, left);

        // Oikealle kanavalle luodaan oma suodatin.
        println!("\n* Creating filter for right channel:");
        let right_filter = filter::Filter::new(self.cutoff_frequency, self.sample_rate, right);

        // Vasen kanava suodatetaan.
        println!("\n* Applying filter on left samples:");
        let left_filtered_samples = left_filter.get_filtered_samples();

        // Oikea kanava suodatetaan.
        println!("\n* Applying filter on right samples:");
        let right_filtered_samples = right_filter.get_filtered_samples();

        // Suodatetut kanavat yhdistetään.
        println!("\n* Joining filtered left and right channels.");
        return  Self::join_channels(left_filtered_samples, right_filtered_samples);
    }

    /// Metodi suodattaa yksikanavaisen äänitiedoston ja palauttaa suodatetut ääninäytteet.
    fn filter_mono(&self) -> Vec<i32> {

        // Suodatin luodaan luetun tiedoston ja annetun ylärajataajuuden perusteella.
        println!("\n* Creating filter.");
        let filter = filter::Filter::new(self.cutoff_frequency, self.sample_rate, self.samples.clone());

        // Ääninäytteet suodatetaan.
        println!("\n* Applying filter on samples:");
        return filter.get_filtered_samples();
    }

    /// Funktio erottelee vasemman ja oikean kanavan ääninäytteistä ja palauttaa ne erillisinä
    /// taulukoina.
    pub fn extract_left_and_right_channels(samples: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
        let left: Vec<i32> = samples.iter().step_by(2).copied().collect();
        let right: Vec<i32> = samples.iter().skip(1).step_by(2).copied().collect();
        (left, right)
    }

    /// Funktio yhdistää vasemman ja oikean kanavan yhdeksi taulukoksi.
    pub fn join_channels(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
        let length = left.len() + right.len();
        let mut result: Vec<i32> = std::vec::from_elem(0, length);
        let mut i: usize = 0;
        let mut j: usize = 0;
        while i < length {
            if i % 2 == 0 {
                result[i] = left[j];
            }
            else {
                result[i] = right[j];
                j += 1;
            }
            i += 1;
        }
        result
    }
}
