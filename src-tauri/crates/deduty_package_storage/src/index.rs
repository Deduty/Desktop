use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use async_std::path::PathBuf;
use async_std::sync::RwLock;

use deduty_package_traits::{ DedutyPackage };
use xresult::{ XError, XReason, XResult };

use crate::storage::DedutyPackageStorage;

pub struct DedutyPackageStorageIndex {
    root: PathBuf,
    storages: HashMap<String, Arc<RwLock<DedutyPackageStorage>>>
}

impl DedutyPackageStorageIndex {
    pub fn new(root: PathBuf) -> Self {
        Self { root, storages: HashMap::new() }
    }

    pub async fn remove(&mut self, id: &String) -> XReason {
        async_std::fs::create_dir_all(self.root.clone())
            .await
            .map_err(|error| XError::from(("Deduty package storage error", error.to_string())))?;
        
        self.storages.remove(id);

        let expected_filepath = self.root.join(format!("{id}.json"));
        if expected_filepath.exists().await {
            return async_std::fs::remove_file(expected_filepath)
                .await
                .map_err(|error| XError::from(("Deduty package storage error", error.to_string())).into());
        }

        Ok(())
    }

    pub async fn storage(&mut self, package: &dyn DedutyPackage) -> XResult<Arc<RwLock<DedutyPackageStorage>>> {
        async_std::fs::create_dir_all(self.root.clone())
            .await
            .map_err(|error| XError::from(("Deduty package storage error", error.to_string())))?;

        match self.storages.get(&package.id()) {
            Some(storage) => Ok(storage.clone()),
            None => {
                let supposed = PathBuf::from_str(&format!("{}.json", package.id())).map_err(|error| XError::from(("Deduty package storage error", error.to_string())))?;
                let storage = Arc::new(RwLock::new(DedutyPackageStorage::from(self.root.join(supposed)).await?));
                self.storages.insert(package.id(), storage.clone());
                Ok(storage)
            }
        }
    }

    pub async fn save(&self) -> XResult<Vec<XReason>> {
        async_std::fs::create_dir_all(self.root.clone())
            .await
            .map_err(|error| XError::from(("Deduty package storage error", error.to_string())))?;

        let mut reasons = vec![];
        for storage in self.storages.values() {
            reasons.push(
                storage
                    .read()
                    .await
                    .save()
                    .await
            );
        }
        Ok(reasons)
    }
}
