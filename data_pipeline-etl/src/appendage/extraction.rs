pub enum ExtractionMethod{
    CSV(),
    JSON(),
}

pub mod extractor{
    use super::*;
    pub struct ExtractionOptions{
        file_type: ExtractionMethod,
    }

    impl ExtractionOptions{
        pub fn new(method:ExtractionMethod) -> ExtractionOptions{
            ExtractionOptions{file_type: method}
        }

        pub fn read(path: &str){
            let file = std::fs::OpenOptions::new().read(true).open(path).unwrap();
        }
    }
}