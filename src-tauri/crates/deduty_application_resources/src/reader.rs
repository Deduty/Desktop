use std::collections::HashMap;

use deduty_package_traits::DedutyFileReader;

pub struct FileReaderIndex {
    index: HashMap<String, Box<dyn DedutyFileReader>>
}

impl FileReaderIndex {
    pub fn new() -> Self {
        Self { index: HashMap::new() }
    }

    pub fn index(&mut self) -> &mut HashMap<String, Box<dyn DedutyFileReader>> {
        &mut self.index
    }
}
