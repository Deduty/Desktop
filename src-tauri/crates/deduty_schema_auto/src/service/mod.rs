use std::collections::HashMap;

use async_std::{
    fs::File,
    io::{ ReadExt, WriteExt },
    path::PathBuf,
    sync::{ Arc, RwLock }
};
use deduty_package::ReadPackage;
use serde::{ Serialize, Deserialize };
use xresult::{ XResult, XReason };

use crate::package::AutoPackage;


mod add;
mod index;
mod statefull;
mod sub;
mod unique;
mod update;
mod web_storage;


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Requirement {
    path: String
}


#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoPackageWebStorage {
    package: HashMap<String, String>,
    lections: HashMap<String, HashMap<String, String>>
}


#[derive(Debug, Default)]
pub struct AutoPackageService {
    packages: RwLock<HashMap<String, Arc<AutoPackage>>>,
    storages: RwLock<HashMap<String, Arc<RwLock<AutoPackageWebStorage>>>>,
    root: RwLock<Option<PathBuf>>
}

impl AutoPackageService {
    async fn ensure_existence(&self, package: &str, lection: Option<&str>) -> XReason {
        let Some(package) = self.packages.read().await.get(package).cloned() else {
            return crate::error::error_err(format!("Package with id `{package}` not found"));
        };

        if let Some(lection) = lection {
            if package.lection(lection).await?.is_none() {
                return crate::error::error_err(format!("Lection with id `{lection}` not found"));
            }
        }

        Ok(())
    }

    /// Tries to get, or loads or inits web storage
    /// Since web storage is lazy, this method must be called each time before access storage
    /// Otherwise, it may leads to overriding an existed, but stored locally, values
    async fn load_web_storage(&self, package: &str) -> XResult<Arc<RwLock<AutoPackageWebStorage>>> {
        if !self.packages.read().await.contains_key(package) {
            return crate::error::error_err(format!("Package with id `{package}` not found"));
        }

        if let Some(storage) = self.storages.read().await.get(package).cloned() {
            return Ok(storage);
        }

        let storage_path = {
            let Some(root) = self.root.read().await.clone() else {
                return crate::error::error_err(format!("Unable to load web storage for `{package}`: Service root is unset"));
            };

            root.join(package).join(format!("{package}.json"))
        };

        if !storage_path.exists().await {
            return Ok(
                self.storages
                    .write()
                    .await
                    .entry(package.to_string())
                    .or_default()
                    .clone()
            );
        }
        
        let mut buffer = Vec::new();
        File::open(&storage_path)
            .await
            .map_err(|error|
                crate::error::error(format!("Unable to open storage file `{storage_path:#?}`: {error}")))?
            .read_to_end(&mut buffer)
            .await
            .map_err(|error|
                crate::error::error(format!("Unable to read storage file `{storage_path:#?}`: {error}")))?;

        let storage: AutoPackageWebStorage = {
            serde_json::from_slice(&buffer)
                .map_err(|error| crate::error::error(format!("Unable to deserialize web storage from `{storage_path:#?}`: {error}")))?
        };

        Ok(
            self.storages
                .write()
                .await
                .entry(package.to_string())
                .or_insert(Arc::new(RwLock::new(storage)))
                .clone()
        )
    }

    async fn save_web_storage(&self, package: &str) -> XReason {
        self.ensure_existence(package, None).await?;

        let Some(storage) = self.storages.read().await.get(package).cloned() else {
            // Storage hasn't changed since not even load
            return Ok(());
        };
        
        let Some(root) = self.root.read().await.clone() else {
            return crate::error::error_err("Unable to open storage file: Service root is unset");
        };

        let storage_path = root.join(package).join(format!("{package}.json"));

        let buffer = {
            serde_json::to_vec(&*storage.read().await)
                .map_err(|error| 
                    crate::error::error(format!("Unable to serialize web storage for package with id `{package}`: {error}")))?
        };

        File::create(&storage_path)
            .await
            .map_err(|error|
                crate::error::error(format!("Unable to open storage file `{storage_path:#?}`: {error}")))?
            .write(&buffer)
            .await
            .map_err(|error|
                crate::error::error(format!("Unable to read storage file `{storage_path:#?}`: {error}")))?;

        Ok(())
    }
}
