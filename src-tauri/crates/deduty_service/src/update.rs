use async_trait::async_trait;
use xresult::XReason;


/// ### Important
/// Type that implements this trait must have internal mutability
///
#[async_trait]
pub trait UpdateService: Send + Sync {
    fn requirements(&self) -> &str;
    async fn update(&self, id: &str, serialized: &str) -> XReason;
}
