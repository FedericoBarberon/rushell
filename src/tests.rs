#[cfg(test)]
pub mod utilities {
    use std::io::Cursor;

    pub struct TestBuffers {
        pub input: Cursor<Vec<u8>>,
        pub output: Vec<u8>,
        pub error: Vec<u8>,
    }

    impl TestBuffers {
        pub fn new(input_buf: Option<Vec<u8>>) -> Self {
            let input_buf = input_buf.unwrap_or_default();

            Self {
                input: Cursor::new(input_buf),
                output: Vec::new(),
                error: Vec::new(),
            }
        }
    }

    pub fn build_args(args: &[&str]) -> Vec<String> {
        args.into_iter().map(|&s| s.to_owned()).collect()
    }
}
