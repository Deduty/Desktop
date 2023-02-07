use async_trait::async_trait;
use xresult::XResult;

use deduty_package::DedutyPackage;


#[async_trait]
pub trait IndexService: Send + Sync {
    async fn all(&self) -> XResult<&dyn Iterator<Item = &String>>;
    async fn has(&self, id: &str) -> XResult<bool>;
    async fn get(&self, id: &str) -> XResult<Option<&dyn DedutyPackage>>;
}
