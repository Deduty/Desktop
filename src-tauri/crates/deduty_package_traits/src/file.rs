use async_trait::async_trait;

use xresult::{ XReason, XResult };


#[async_trait]
pub trait DedutyFileReader: Sync + Send {
    async fn next(&mut self, chunk: usize) -> XResult<Option<Vec<u8>>>;
    async fn close(&mut self) -> XReason;
    fn closed(&self) -> bool;
}


#[async_trait]
pub trait DedutyFile: Sync + Send {
    fn id(&self) -> String;
    fn alias(&self) -> Option<String>;
    fn extension(&self) -> String;
    async fn load(&self) -> XResult<Box<dyn DedutyFileReader>>;
}


#[async_trait]
pub trait DedutyFileCollection: Sync + Send {
    async fn alias(&self, alias: &String) -> XResult<Option<&dyn DedutyFile>>;
    async fn file(&self, id: &String) -> XResult<Option<&dyn DedutyFile>>;
    async fn files(&self) -> XResult<Vec<&dyn DedutyFile>>;
}
