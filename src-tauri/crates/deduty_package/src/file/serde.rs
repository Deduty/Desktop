use serde::Serialize;
use xresult::XResult;

#[derive(Serialize)]
pub struct DedutyFile {
    pub alias: Option<String>,
    pub extension: String,
    pub id: String
}

impl DedutyFile {
    pub async fn try_from(object: &dyn crate::file::traits::DedutyFile) -> XResult<DedutyFile> {
        Ok(DedutyFile {
            alias: object.alias(),
            extension: object.extension(),
            id: object.id().to_string()
        })
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
