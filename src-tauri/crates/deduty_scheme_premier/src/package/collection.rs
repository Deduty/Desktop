use std::collections::HashMap;

use async_trait::async_trait;
use uuid::Uuid;

use deduty_package_traits::{
    DedutyFile,
    DedutyFileCollection
};
use xresult::{ XError, XResult };

use crate::file::PremierFile;

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

    async fn file(&self, id: &String) -> XResult<Option<&dyn DedutyFile>> {
        /* Premier scheme use uuid, so we can perform validation check */
        Uuid::parse_str(id.as_str())
            .map_err(|error| XError::from(("Premier scheme error", error.to_string())))?;

        Ok(
            self.collection
                .values()
                .find(|file| file.id() == *id)
                .map(|object| object as &dyn DedutyFile)
        )
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
