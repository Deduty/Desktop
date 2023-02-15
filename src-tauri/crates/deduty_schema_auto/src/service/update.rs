use async_std::{
    path::PathBuf,
    sync::Arc
};
use async_trait::async_trait;
use deduty_service::UpdateService;
use xresult::XReason;

use crate::package::AutoPackage;

use super::{ AutoPackageService, Requirement };


#[async_trait]
impl UpdateService for AutoPackageService {
    fn requirements(&self) -> &str {
        r#"{ "path": "DirectoryPath" }"#
    }

    async fn update(&self, id: &str, serialized: &str) -> XReason {
        let path = serde_json::from_str::<Requirement>(serialized)
            .map_err(|error|
                crate::error::error(format!("Unable deserialize update requirements: {error}")))?
            .path;
        
        let Some(package) = self.packages.write().await.remove(id) else {
            return crate::error::error_err(format!("Package with id `{id}` not found"));
        };

        // Init storage if it exist -> Save web data in memory
        let _ = self.load_web_storage(id).await;
        let Some(root) = self.root.read().await.clone() else {
            return crate::error::error_err(format!("Unable to remove old package for `{id}`: Service root is unset"));
        };

        let mut updated = AutoPackage::load(PathBuf::from(path)).await?;
        updated.update(&package).await;
        package.remove(&root).await?;
        self.packages.write().await.insert(id.to_string(), Arc::new(updated));

        Ok(())
    }
}
