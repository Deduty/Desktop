use async_trait::async_trait;
use deduty_service::SubService;
use xresult::XReason;

use super::AutoPackageService;


#[async_trait]
impl SubService for AutoPackageService {
    async fn sub(&self, id: &str) -> XReason {
        let Some(package) = self.packages.write().await.remove(id) else {
            return crate::error::error_err(format!("Package with id `{id}` not found"));
        };

        // Remove storage in memory
        self.storages.write().await.remove(id);

        let Some(root) = self.root.read().await.clone() else {
            return Ok(());
        };

        // WebStorage exist under package directory, so it will be deleted in remove
        package.remove(&root).await
    }
}
