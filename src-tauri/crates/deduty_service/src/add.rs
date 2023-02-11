use async_trait::async_trait;
use xresult::XResult;

use deduty_package::DedutyPackage;

use crate::Borrowed;


/// ### Important
/// Type that implements this trait must have internal mutability
///
#[async_trait]
pub trait AddService: Send + Sync {
    fn requirements(&self) -> &str;
    async fn add(&self, serialized: &str) -> XResult<Borrowed<dyn DedutyPackage>>;
}
