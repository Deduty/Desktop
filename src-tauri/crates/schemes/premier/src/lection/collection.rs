use async_trait::async_trait;

use package::file::traits::{
    DedutyFile,
    DedutyFileCollection
};

use crate::error::XResult;

use crate::file::PremierFile;

pub struct PremierLectionFileCollection {
    collection: Vec<PremierFile>
}

impl PremierLectionFileCollection {
    pub fn new() -> Self {
        Self { collection: Vec::new() }
    }

    pub fn from(vector: Vec<PremierFile>) -> Self {
        Self { collection: vector }
    }
}

#[async_trait]
impl DedutyFileCollection for PremierLectionFileCollection {
    async fn alias(&self, alias: &String) -> XResult<Option<&dyn DedutyFile>> {
        Ok(
            self.collection
                .get(alias.parse::<usize>()?)
                .map(|file| file as &dyn DedutyFile)
        )
    }

    async fn file(&self, location: &async_std::path::Path) -> XResult<Option<&dyn DedutyFile>> {
        for file in &self.collection {
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
                .map(|file| file as &dyn DedutyFile)
                .collect()
        )
    }
}
