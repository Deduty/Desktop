use async_std::sync::Arc;
use async_trait::async_trait;
use deduty_package::{ PeekLection, SerdeFile };
use xresult::XResult;

use crate::borrowed::{ BorrowedIterator, BorrowedItem };

use super::AutoLection;


/// This trait is used when application needs to serialize or calculate file size.
/// Must be **cached** (if possible) and **can be used very often**.
#[async_trait]
impl PeekLection for AutoLection {
    async fn files(&self) -> XResult<BorrowedIterator<dyn SerdeFile>> {
        Ok(Box::new(
            self.files_order
                .iter()
                .map(|file| BorrowedItem::boxed(file.clone() as Arc<dyn SerdeFile>))
                .collect::<Vec<_>>()
                .into_iter()
        ))
    }
}
