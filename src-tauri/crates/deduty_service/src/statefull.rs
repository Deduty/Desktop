use async_std::path::PathBuf;
use async_trait::async_trait;
use xresult::XResult;

/// ### Important
/// Type that implements this trait must have internal mutability
///
#[async_trait]
pub trait StateFullService: Send + Sync {
    async fn get_root(&self) -> XResult<Option<PathBuf>>;
    async fn set_root(&self, root: PathBuf) -> XResult<Option<PathBuf>>;

    async fn load_all(&self, root: &PathBuf) -> XResult<Box<dyn Iterator<Item = XResult<String>> + Send>>;
    async fn save_all(&self, root: &PathBuf) -> XResult<Box<dyn Iterator<Item = XResult<String>> + Send>>;
}
