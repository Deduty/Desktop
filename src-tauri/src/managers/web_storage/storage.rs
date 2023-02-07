use std::collections::HashMap;
use std::sync::Arc;

use async_std::path::PathBuf;
use async_std::sync::RwLock;

use xresult::{ XError, XReason, XResult };

use super::scheme::WebStorageScheme;


pub struct WebStorageManager {
    root: PathBuf,
    storages: RwLock<HashMap<String, Arc<RwLock<WebStorageScheme>>>>
}

impl WebStorageManager {
    pub fn new(root: PathBuf) -> Self {
        Self { root, storages: Default::default() }
    }

    pub async fn remove(&mut self, package: &str) -> XReason {
        async_std::fs::create_dir_all(self.root.clone())
            .await
            .map_err(|error| XError::from(("Storage error", error.to_string())))?;
        
        self.storages
            .write()
            .await
            .remove(package);

        let expected_filepath = self.root.join(format!("{package}.json"));
        if expected_filepath.exists().await {
            return async_std::fs::remove_file(expected_filepath)
                .await
                .map_err(|error| XError::from(("Storage error", error.to_string())).into());
        }

        Ok(())
    }

    pub async fn storage(&self, package: &str) -> XResult<Arc<RwLock<WebStorageScheme>>> {
        async_std::fs::create_dir_all(self.root.clone())
            .await
            .map_err(|error| XError::from(("Storage error", error.to_string())))?;

        if let Some(storage) = self.storages.write().await.get(package) {
            return Ok(storage.clone());
        }

        let storage = WebStorageScheme::from(self.root.join(format!("{package}.json"))).await?;

        Ok(
            self.storages
                .write()
                .await
                .entry(package.to_string())
                .or_insert(Arc::new(RwLock::new(storage)))
                .clone()
        )
    }

    pub async fn save(&self) -> XResult<Vec<XReason>> {
        async_std::fs::create_dir_all(self.root.clone())
            .await
            .map_err(|error| XError::from(("Storage error", error.to_string())))?;

        let mut reasons = vec![];
        for (id, storage) in self.storages.read().await.iter() {
            reasons.push(storage.read().await.save(self.root.join(id)).await);
        }
        Ok(reasons)
    }
}
