use async_std::sync::Arc;
use async_trait::async_trait;
use deduty_package::{ ReadPackage, DedutyLection };
use xresult::XResult;

use crate::borrowed::{ Borrowed, BorrowedItem, BorrowedIterator };

use super::AutoPackage;

/// This expensive trait will be used when application needs
///     to read some or all files when frontend needs to obtain
///     files ids must be cached if possible
#[async_trait]
impl ReadPackage for AutoPackage {
    async fn lection(&self, id: &str) -> XResult<Option<Borrowed<dyn DedutyLection>>> {
        Ok(
            self.lections_index
                .get(id)
                .map(|lection| BorrowedItem::boxed(lection.clone() as Arc<dyn DedutyLection>))
        )
    }

    async fn lections(&self) -> XResult<BorrowedIterator<dyn DedutyLection>> {
        Ok(Box::new(
            self.lections_order
                .iter()
                .map(|lection| BorrowedItem::boxed(lection.clone() as Arc<dyn DedutyLection>))
                .collect::<Vec<_>>()
                .into_iter()
        ))
    }
}
