use async_trait::async_trait;
use xresult::{ XReason, XResult };


#[async_trait]
pub trait DedutyFileReader: Sync + Send {
    async fn closed(&self) -> XResult<bool>;

    async fn close(&self) -> XReason;
    async fn next(&self, chunk: usize) -> XResult<Option<Vec<u8>>>;
}
