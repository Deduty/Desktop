use async_trait::async_trait;

use serde::Serialize;
use xresult::XResult;

use crate::lection::SerdeLection;
use super::traits::AsyncTrySerialize;
use super::file::DedutyFileSerde;


#[derive(Serialize)]
pub struct DedutyLectionSerde {
    id: String,
    meta: Option<String>,
    files: Vec<DedutyFileSerde>,
    size: Option<usize>
}

#[async_trait]
impl AsyncTrySerialize<DedutyLectionSerde> for &dyn SerdeLection {
    async fn try_serde(&self) -> XResult<DedutyLectionSerde> {
        let mut files = Vec::new();

        for file in self.files().await? {
            files.push(file.try_serde().await?);
        }

        Ok(
            DedutyLectionSerde {
                id: self.id().to_string(),
                meta: self.meta().map(str::to_string),
                files,
                size: self.size()
            }
        )
    }
}
