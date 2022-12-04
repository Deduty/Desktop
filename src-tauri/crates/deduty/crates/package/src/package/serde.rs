use serde::Serialize;

use crate::error::XResult;
use crate::file::serde::DedutyFileCollection;


#[derive(Serialize)]
pub struct DedutyPackageMeta {
    pub name: String,
    pub version: String,
    pub language: String,
    pub tags: Vec<String>
}

impl DedutyPackageMeta {
    pub async fn try_from(object: &dyn crate::package::traits::DedutyPackageMeta) -> XResult<DedutyPackageMeta> {
        Ok(
            DedutyPackageMeta {
                name: object.name().clone(),
                version: object.version().clone(),
                language: object.language().clone(),
                tags: object.tags().clone()
            }
        )
    }
}


#[derive(Serialize)]
pub struct DedutyPackage {
    pub id: String,
    pub meta: DedutyPackageMeta,
    pub files: DedutyFileCollection
}


impl DedutyPackage {
    pub async fn try_from(object: &dyn crate::package::traits::DedutyPackage) -> XResult<DedutyPackage> {
        Ok(
            DedutyPackage {
                id: object.id().to_string(),
                meta: DedutyPackageMeta::try_from(object.meta()).await?,
                files: DedutyFileCollection::try_from(object.files()).await?,
            }
        )
    }
}
