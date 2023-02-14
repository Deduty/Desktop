use async_trait::async_trait;
use xresult::XResult;

use crate::{ BorrowedIterator, SerdeLection };


/// This cheap trait will be used each time,
///     when frontend needs to obtain lection id
///     or get whole package size must be cached if possible
#[async_trait]
pub trait PeekPackage: Sync + Send {
    async fn lections(&self) -> XResult<BorrowedIterator<dyn SerdeLection>>;
}
