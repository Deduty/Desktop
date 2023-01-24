use serde::{ Deserialize, Serialize };

use deduty_package_traits::{
    DedutyPackage,
    DedutyPackageMeta
};
use xresult::XResult;

use crate::SerdeDedutyFileCollection;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SerdeDedutyPackageMeta {
    pub name: String,
    pub version: String,
    pub language: String,
    pub tags: Vec<String>
}

impl SerdeDedutyPackageMeta {
    pub async fn try_from(object: &dyn DedutyPackageMeta) -> XResult<Self> {
        Ok(
            Self {
                name: object.name(),
                version: object.version(),
                language: object.language(),
                tags: object.tags()
            }
        )
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SerdeDedutyPackage {
    pub id: String,
    pub meta: SerdeDedutyPackageMeta,
    pub files: SerdeDedutyFileCollection
}

impl SerdeDedutyPackage {
    pub async fn try_from(object: &dyn DedutyPackage) -> XResult<Self> {
        Ok(
            Self {
                id: object.id().to_string(),
                meta: SerdeDedutyPackageMeta::try_from(object.meta()).await?,
                files: SerdeDedutyFileCollection::try_from(object.files()).await?,
            }
        )
    }
}
