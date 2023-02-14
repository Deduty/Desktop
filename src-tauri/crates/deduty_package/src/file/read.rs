use async_trait::async_trait;
use xresult::XResult;

use crate::reader::DedutyFileReader;


#[async_trait]
pub trait ReadFile: Sync + Send {
    async fn reader(&self) -> XResult<Box<dyn DedutyFileReader>>;
}
