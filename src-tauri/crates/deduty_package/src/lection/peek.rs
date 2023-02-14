use async_trait::async_trait;
use xresult::XResult;

use crate::{ BorrowedIterator, SerdeFile };


/// This trait is used when application needs to serialize or calculate file size.
/// Must be **cached** (if possible) and **can be used very often**.
#[async_trait]
pub trait PeekLection: Sync + Send {
    async fn files(&self) -> XResult<BorrowedIterator<dyn SerdeFile>>;
}
