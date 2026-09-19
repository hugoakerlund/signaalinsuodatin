use signaalinsuodatin::filter;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn low_pass_filter_is_created() {
        let samples: Vec<i32> = std::vec::from_elem(1000, 100);
        let sample_rate: f64 = 48000.0;
        let cutoff_frequency: f64 = 100.0;
        let filter = filter::Filter::new(cutoff_frequency, sample_rate, samples);
        let lpf = filter.get_lpf();
        assert_eq!(lpf.len(), 128);
    }
}
