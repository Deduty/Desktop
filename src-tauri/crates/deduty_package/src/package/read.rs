use async_trait::async_trait;
use xresult::XResult;

use crate::{ Borrowed, BorrowedIterator, DedutyLection };


/// This expensive trait will be used when application needs
///     to read some or all files when frontend needs to obtain
///     files ids must be cached if possible
#[async_trait]
pub trait ReadPackage: Sync + Send {
    async fn lection(&self, id: &str) -> XResult<Option<Borrowed<dyn DedutyLection>>>;
    async fn lections(&self) -> XResult<BorrowedIterator<dyn DedutyLection>>;
}

