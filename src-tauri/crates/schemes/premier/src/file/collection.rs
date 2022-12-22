use std::collections::HashMap;

use async_trait::async_trait;

use package::file::traits::{
    DedutyFile,
    DedutyFileCollection
};

use crate::error::XResult;

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

#[async_trait]
impl DedutyFileCollection for PremierPackageFileCollection {
    async fn alias(&self, alias: &String) -> XResult<Option<&dyn DedutyFile>> {
        Ok(
            self.collection
                .get(alias)
                .map(|file| file as &dyn DedutyFile)
        )
    }

    async fn file(&self, location: &async_std::path::Path) -> XResult<Option<&dyn DedutyFile>> {
        for file in self.collection.values() {
            if file.location().await?.as_path() == location {
                return Ok(Some(file))
            }
        }
        Ok(None)
    }

    async fn files(&self) -> XResult<Vec<&dyn DedutyFile>> {
        Ok(
            self.collection
                .iter()
                .map(|(_, file)| file as &dyn DedutyFile)
                .collect()
        )
    }
}
