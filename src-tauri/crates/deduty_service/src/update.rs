use async_trait::async_trait;
use xresult::XReason;


/// ### Important
/// Type that implements this trait must have internal mutability
///
#[async_trait]
pub trait UpdateService: Send + Sync {
    fn requirements(&self) -> &String;
    async fn update(&self, serialized: String, id: &str) -> XReason;
}
