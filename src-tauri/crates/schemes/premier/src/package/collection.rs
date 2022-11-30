use std::collections::HashMap;

use package::file::traits::{
    DedutyFile,
    DedutyFileCollection
};

use super::file::PremierFile;

pub struct PremierPackageFileCollection {
    collection: HashMap<String, PremierFile>
}

impl PremierPackageFileCollection {
    pub fn new() -> Self {
        Self { collection: HashMap::new() }
    }

    pub fn from(map: HashMap<String, PremierFile>) -> Self {
        Self { collection: map }
    }
}

impl DedutyFileCollection for PremierPackageFileCollection {
    fn alias(&self, alias: &String) -> Option<&dyn DedutyFile> {
        self.collection
            .get(alias)
            .map(|file| file as &dyn DedutyFile)
    }

    fn file(&self, location: &async_std::path::Path) -> Option<&dyn DedutyFile> {
        self.collection
            .values()
            .find(|&file| file.location().as_path() == location)
            .map(|file| file as &dyn DedutyFile)
    }

    fn files(&self) -> Vec<&dyn DedutyFile> {
        self.collection
            .iter()
            .map(|(_, file)| file as &dyn DedutyFile)
            .collect()
    }
}
