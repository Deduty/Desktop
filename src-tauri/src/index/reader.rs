use std::collections::HashMap;

use uuid::Uuid;

use deduty_package::file::traits::DedutyFileReader;

pub struct FileReaderIndex {
    index: HashMap<Uuid, Box<dyn DedutyFileReader>>
}

impl FileReaderIndex {
    pub fn new() -> Self {
        Self { index: HashMap::new() }
    }

    pub fn index(&mut self) -> &mut HashMap<Uuid, Box<dyn DedutyFileReader>> {
        &mut self.index
    }
}
