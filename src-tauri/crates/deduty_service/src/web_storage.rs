use async_trait::async_trait;
use xresult::XResult;


#[async_trait]
pub trait WebStorageService: Send + Sync {
    async fn export(&self, package: &str, path: &str) -> XResult<()>;

    async fn import(&self, package: &str, path: &str) -> XResult<()>;

    async fn clear(&self, package: &str) -> XResult<()>;

    async fn delete(&self, package: &str, lection: Option<&str>, key: &str) -> XResult<Option<String>>;

    async fn get(&self, package: &str, lection: Option<&str>, key: &str, fallback: Option<&str>) -> XResult<Option<String>>;

    async fn set(&self, package: &str, lection: Option<&str>, key: &str, value: &str, replaced: bool) -> XResult<Option<String>>;
}
