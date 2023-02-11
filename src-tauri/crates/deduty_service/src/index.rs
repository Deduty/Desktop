use async_trait::async_trait;
use xresult::XResult;

use deduty_package::DedutyPackage;
use crate::{ Borrowed, BorrowedIterator };


#[async_trait]
pub trait IndexService: Send + Sync {
    async fn all(&self) -> XResult<BorrowedIterator<dyn DedutyPackage>>;
    async fn has(&self, id: &str) -> XResult<bool>;
    async fn get(&self, id: &str) -> XResult<Option<Borrowed<dyn DedutyPackage>>>;
}
