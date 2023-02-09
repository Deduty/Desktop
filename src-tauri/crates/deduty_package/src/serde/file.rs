use async_trait::async_trait;

use serde::Serialize;
use xresult::XResult;

use crate::file::SerdeFile;
use super::AsyncTrySerialize;


#[derive(Serialize)]
pub struct DedutyFileSerde {
    id: String,
    ext: String,
    meta: Option<String>,
    size: Option<usize>
}

#[async_trait]
impl AsyncTrySerialize<DedutyFileSerde> for &dyn SerdeFile {
    async fn try_serde(&self) -> XResult<DedutyFileSerde> {
        Ok(
            DedutyFileSerde {
                id: self.id().clone(),
                ext: self.ext().clone(),
                meta: self.meta().cloned(),
                size: self.size()
            }
        )
    }
}
