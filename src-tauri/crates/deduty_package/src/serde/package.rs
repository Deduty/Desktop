use async_trait::async_trait;
use serde::Serialize;
use xresult::XResult;

use crate::package::SerdePackage;

use super::traits::AsyncTrySerialize;
use super::lection::DedutyLectionSerde;


#[derive(Serialize)]
pub struct DedutyPackageSerde {
    id: String,
    meta: Option<String>,
    lections: Vec<DedutyLectionSerde>,
    size: Option<usize>
}


#[async_trait]
impl AsyncTrySerialize<DedutyPackageSerde> for &dyn SerdePackage {
    async fn try_serde(&self) -> XResult<DedutyPackageSerde> {
        let mut lections = Vec::new();
        for lection in self.lections().await? {
            lections.push(lection.try_serde().await?);
        }

        Ok(
            DedutyPackageSerde {
                id: self.id().to_string(),
                meta: self.meta().map(str::to_string),
                lections,
                size: self.size()
            }
        )
    }
}
