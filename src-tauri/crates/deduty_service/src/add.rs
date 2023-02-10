use async_trait::async_trait;
use xresult::XResult;


use deduty_package::DedutyPackage;


/// ### Important
/// Type that implements this trait must have internal mutability
///
#[async_trait]
pub trait AddService: Send + Sync {
    fn requirements(&self) -> &String;
    async fn add(&self, serialized: &str) -> XResult<&dyn DedutyPackage>;
}
