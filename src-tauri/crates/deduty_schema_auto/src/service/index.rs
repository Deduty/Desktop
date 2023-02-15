use async_std::sync::Arc;
use async_trait::async_trait;
use deduty_package::DedutyPackage;
use deduty_service::IndexService;
use xresult::XResult;

use crate::borrowed::{ Borrowed, BorrowedItem, BorrowedIterator };

use super::AutoPackageService;


#[async_trait]
impl IndexService for AutoPackageService {
    async fn all(&self) -> XResult<BorrowedIterator<dyn DedutyPackage>> {
        Ok(Box::new(
            self.packages
                .read()
                .await
                .clone()
                .into_values()
                .map(|package| package as Arc<dyn DedutyPackage>)
                .map(BorrowedItem::boxed)
        ))
    }

    async fn has(&self, id: &str) -> XResult<bool> {
        Ok(self.packages.read().await.contains_key(id))
    }

    async fn get(&self, id: &str) -> XResult<Option<Borrowed<dyn DedutyPackage>>> {
        Ok(
            self.packages
                .read()
                .await
                .get(id)
                .map(|package| BorrowedItem::boxed(package.clone() as Arc<dyn DedutyPackage>))
        )
    }
}
