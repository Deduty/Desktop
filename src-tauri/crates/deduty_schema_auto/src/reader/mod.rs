use async_std::{
    io::BufReader,
    fs::File,
    sync::Mutex
};

#[allow(clippy::module_inception)]
mod reader;

#[derive(Debug)]
pub struct AutoFileReader {
    reader: Mutex<Option<BufReader<File>>>
}

impl AutoFileReader {
    pub fn new(file: File) -> Self {
        Self { reader: Mutex::new(Some(BufReader::new(file))) }
    }
}
