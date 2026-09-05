use signaalinsuodatin::fft;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn tests_are_working() -> () {
        let result: String = fft::test_function();
        assert_eq!(result, "working!")
    }

}

