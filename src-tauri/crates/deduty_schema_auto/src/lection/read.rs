use async_std::sync::Arc;
use async_trait::async_trait;
use deduty_package::{ ReadLection, DedutyFile };
use xresult::XResult;

use crate::borrowed::{ Borrowed, BorrowedIterator, BorrowedItem };
use super::AutoLection;


/// This trait is used when application needs to read file or make expensive operation.
#[async_trait]
impl ReadLection for AutoLection {
    async fn file(&self, id: &str) -> XResult<Option<Borrowed<dyn DedutyFile>>> {
        Ok(
            self.files_index
                .get(id)
                .map(|file| BorrowedItem::boxed(file.clone() as Arc<dyn DedutyFile>))
        )
    }

    async fn files(&self) -> XResult<BorrowedIterator<dyn DedutyFile>> {
        Ok(Box::new(
            self.files_order
                .iter()
                .map(|file| BorrowedItem::boxed(file.clone() as Arc<dyn DedutyFile>))
                .collect::<Vec<_>>()
                .into_iter()
        ))
    }
}
