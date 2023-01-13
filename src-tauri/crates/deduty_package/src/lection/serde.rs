use serde::Serialize;

use xresult::XResult;
use crate::file::serde::DedutyFileCollection;


#[derive(Serialize)]
pub struct DedutyLectionMeta {
    pub name: String,
    pub order: u64
}

impl DedutyLectionMeta {
    pub async fn try_from(object: &dyn crate::lection::traits::DedutyLectionMeta) -> XResult<DedutyLectionMeta> {
        Ok(
            DedutyLectionMeta {
                name: object.name().clone(),
                order: object.order()
            }
        )
    }
}


#[derive(Serialize)]
pub struct DedutyLection {
    pub id: String,
    pub meta: DedutyLectionMeta,
    pub files: DedutyFileCollection
}

impl DedutyLection {
    pub async fn try_from(object: &dyn crate::lection::traits::DedutyLection) -> XResult<DedutyLection> {
        Ok(
            DedutyLection {
                id: object.id().to_string(),
                meta: DedutyLectionMeta::try_from(object.meta()).await?,
                files: DedutyFileCollection::try_from(object.files()).await?,
            }
        )
    }
}
