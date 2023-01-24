use serde::{ Deserialize, Serialize };

use deduty_package_traits::{
    DedutyFile,
    DedutyFileCollection
};
use xresult::XResult;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SerdeDedutyFile {
    pub alias: Option<String>,
    pub extension: String,
    pub id: String
}

impl SerdeDedutyFile {
    pub async fn try_from(file: &dyn DedutyFile) -> XResult<Self> {
        Ok(Self {
            alias: file.alias(),
            extension: file.extension(),
            id: file.id()
        })
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SerdeDedutyFileCollection {
    pub files: Vec<SerdeDedutyFile>
}

impl SerdeDedutyFileCollection {
    pub async fn try_from<'s>(collection: &'s dyn DedutyFileCollection) -> XResult<Self> {
        let mut files = vec![];

        for file in collection.files().await? {
            files.push(SerdeDedutyFile::try_from(file).await?)
        }

        Ok(Self { files })
    }
}
