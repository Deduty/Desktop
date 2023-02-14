use async_trait::async_trait;
use xresult::XResult;

use crate::{ Borrowed, BorrowedIterator, DedutyFile };


/// This trait is used when application needs to read file or make expensive operation.
#[async_trait]
pub trait ReadLection: Sync + Send {
    async fn file(&self, id: &str) -> XResult<Option<Borrowed<dyn DedutyFile>>>;
    async fn files(&self) -> XResult<BorrowedIterator<dyn DedutyFile>>;
}
