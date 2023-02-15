use async_std::{
    path::PathBuf,
    stream::StreamExt,
    sync::Arc
};
use async_trait::async_trait;
use deduty_package::UniquePackage;
use deduty_service::StateFullService;
use xresult::XResult;

use crate::package::AutoPackage;

use super::AutoPackageService;


#[async_trait]
impl StateFullService for AutoPackageService {
    async fn get_root(&self) -> XResult<Option<PathBuf>> {
        Ok(self.root.read().await.clone())
    }

    async fn set_root(&self, root: PathBuf) -> XResult<Option<PathBuf>> {
        Ok(self.root.write().await.replace(root.clone()))
    }

    async fn load_all(&self, root: &PathBuf) -> XResult<Box<dyn Iterator<Item = XResult<String>> + Send>> {
        let mut results: Vec<XResult<String>> = Vec::new();

        let mut packages = self.packages.write().await;

        let mut entries = async_std::fs::read_dir(root.clone())
            .await
            .map_err(|error|
                crate::error::error(format!("Unable to scan directory {root:#?}: {error}")))?;

        while let Some(entry) = entries.next().await {
            let lection_path = match entry.map(|entry| entry.path()) {
                Ok(exact) => exact,
                Err(error) => {
                    results.push(crate::error::error_err(format!("Unable to get directory entry: {error}")));
                    continue;
                }
            };

            match AutoPackage::load(lection_path).await {
                Ok(package) => {
                    match packages.contains_key(package.id()) {
                        true => results.push(
                            crate::error::error_err(format!("Unable load package with occupied id `{}`", package.id()))),
                        false => {
                            results.push(Ok(package.id().to_string()));
                            packages.insert(package.id().to_string(), Arc::new(package));
                        }
                    }
                },
                Err(error) => results.push(Err(error))
            }
        }

        Ok(Box::new(results.into_iter()))
    }

    async fn save_all(&self, root: &PathBuf) -> XResult<Box<dyn Iterator<Item = XResult<String>> + Send>> {
        let mut results: Vec<XResult<String>> = Vec::new();

        for package in self.packages.read().await.values() {
            // Save storage error in not critical
            let _ = self.save_web_storage(package.id()).await;
            results.push(package.save(root).await.map(|_| package.id().to_string()));
        }

        Ok(Box::new(results.into_iter()))
    }
}
