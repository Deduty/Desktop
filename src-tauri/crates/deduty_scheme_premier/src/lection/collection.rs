use async_trait::async_trait;
use uuid::Uuid;

use deduty_package_traits::{
    DedutyFile,
    DedutyFileCollection
};
use xresult::{ XError, XResult };

use crate::file::PremierFile;

#[derive(Debug, Clone)]
pub struct PremierLectionFileCollection {
    pub(crate) collection: Vec<PremierFile>
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

    async fn file(&self, id: &String) -> XResult<Option<&dyn DedutyFile>> {
        /* Premier scheme use uuid, so we can perform validation check */
        Uuid::parse_str(id.as_str())
            .map_err(|error| XError::from(("Premier scheme error", error.to_string())))?;

        Ok(
            self.collection
                .iter()
                .find(|file| file.id() == *id)
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
