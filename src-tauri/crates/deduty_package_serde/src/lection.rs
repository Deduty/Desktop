use serde::{ Deserialize, Serialize };

use deduty_package_traits::{
    DedutyLectionMeta,
    DedutyLection
};
use xresult::XResult;

use crate::file::SerdeDedutyFileCollection;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SerdeDedutyLectionMeta {
    pub name: String,
    pub order: u64
}

impl SerdeDedutyLectionMeta {
    pub async fn try_from(meta: &dyn DedutyLectionMeta) -> XResult<Self> {
        Ok(
            Self {
                name: meta.name(),
                order: meta.order()
            }
        )
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SerdeDedutyLection {
    pub id: String,
    pub meta: SerdeDedutyLectionMeta,
    pub files: SerdeDedutyFileCollection
}

impl SerdeDedutyLection {
    pub async fn try_from(lection: &dyn DedutyLection) -> XResult<Self> {
        Ok(
            Self {
                id: lection.id().clone(),
                meta: SerdeDedutyLectionMeta::try_from(lection.meta()).await?,
                files: SerdeDedutyFileCollection::try_from(lection.files()).await?,
            }
        )
    }
}
