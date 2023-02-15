use async_std::sync::Arc;
use async_trait::async_trait;
use deduty_package::{ PeekPackage, SerdeLection };
use xresult::XResult;

use crate::borrowed::{ BorrowedItem, BorrowedIterator };
use super::AutoPackage;


/// This cheap trait will be used each time,
///     when frontend needs to obtain lection id
///     or get whole package size must be cached if possible
#[async_trait]
impl PeekPackage for AutoPackage {
    async fn lections(&self) -> XResult<BorrowedIterator<dyn SerdeLection>> {
        Ok(Box::new(
            self.lections_order
                .iter()
                .map(|lection| BorrowedItem::boxed(lection.clone() as Arc<dyn SerdeLection>))
                .collect::<Vec<_>>()
                .into_iter()
        ))
    }
}
