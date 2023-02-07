use async_trait::async_trait;
use xresult::XReason;


/// ### Important
/// Type that implements this trait must have internal mutability
///
#[async_trait]
pub trait SubService: Send + Sync {
    // TODO: Make same as AddService?
    async fn sub(&self, id: &str) -> XReason;
}
