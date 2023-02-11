use async_std::path::PathBuf;
use async_trait::async_trait;
use xresult::{ XReason, XResult };

/// ### Important
/// Type that implements this trait must have internal mutability
///
#[async_trait]
pub trait StateFullService: Send + Sync {
    async fn load_all(&self, root: &PathBuf) -> XResult<Box<dyn Iterator<Item = XReason> + Send>>;
    async fn save_all(&self, root: &PathBuf) -> XResult<Box<dyn Iterator<Item = XReason> + Send>>;
}
