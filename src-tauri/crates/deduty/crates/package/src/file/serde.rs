use serde::Serialize;

use crate::error::{ PackageError, XResult };


#[derive(Serialize)]
pub struct DedutyFile {
    pub alias: Option<String>,
    pub location: String,
    pub extension: String
}

impl DedutyFile {
    pub async fn try_from(object: &dyn crate::file::traits::DedutyFile) -> XResult<DedutyFile> {
        let alias = object.alias().cloned();
        let extension = object.extension().clone();
        let location = match object.location().await?.clone().to_str() {
            Some(value) => value.to_string(),
            None => return Err(
                Box::new(
                    PackageError::new(
                        "Serialization error: Unable convert location into UTF-8 string".to_string())))
        };

        Ok(DedutyFile { alias, location, extension })
    }
}


#[derive(Serialize)]
pub struct DedutyFileCollection {
    pub files: Vec<DedutyFile>
}

impl DedutyFileCollection {
    pub async fn try_from<'s>(object: &'s dyn crate::file::traits::DedutyFileCollection) -> XResult<Self> {
        let mut files = vec![];
        let t = object.files::<'s, 's>();

        for file in t.await?.into_iter() {
            files.push(DedutyFile::try_from(file).await?)
        }

        Ok(Self { files })
    }
}
