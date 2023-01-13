use async_trait::async_trait;
use uuid::Uuid;

use deduty_package::file::traits::{
    DedutyFile,
    DedutyFileCollection
};
use xresult::XResult;

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

    async fn file(&self, uuid: &Uuid) -> XResult<Option<&dyn DedutyFile>> {
        Ok(
            self.collection
                .iter()
                .find(|file| file.id() == *uuid)
                .map(|object| object as &dyn DedutyFile)
        )
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
