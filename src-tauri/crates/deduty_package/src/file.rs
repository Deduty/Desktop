use async_trait::async_trait;
use xresult::{ XReason, XResult };


#[async_trait]
pub trait DedutyFileReader: Sync + Send {
    fn closed(&self) -> bool;

    async fn close(&mut self) -> XReason;
    async fn next(&mut self, chunk: usize) -> XResult<Option<Vec<u8>>>;
}


#[async_trait]
pub trait DedutyFile: Sync + Send {
    fn id(&self) -> &String;
    fn ext(&self) -> &String;

    async fn load(&self) -> XResult<Box<dyn DedutyFileReader>>;
}
