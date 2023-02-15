use async_std::{
    fs::File,
    io::{ WriteExt, ReadExt },
    path::PathBuf,
    sync::{ Arc, RwLock }
};
use async_trait::async_trait;
use deduty_service::WebStorageService;
use xresult::{ XResult, XReason };

use super::{ AutoPackageService, AutoPackageWebStorage };


#[async_trait]
impl WebStorageService for AutoPackageService {
    async fn export(&self, package: &str, path: &str) -> XReason {
        let storage = self.load_web_storage(package).await?;

        let content = serde_json::ser::to_vec(&*storage.read().await)
            .map_err(|error|
                crate::error::error(format!("Unable to serialize storage object: {error}")))?;

        File::create(&path)
            .await
            .map_err(|error|
                crate::error::error(format!("Unable to create file at `{path:#?}`: {error}")))?
            .write_all(content.as_slice())
            .await
            .map_err(|error|
                crate::error::error(format!("Unable to write in file at `{path:#?}`: {error}")))?;

        Ok(())
    }

    async fn import(&self, package: &str, path: &str) -> XReason {
        let mut buffer = Vec::new();
        File::open(PathBuf::from(path))
            .await
            .map_err(|error|
                crate::error::error(format!("Unable to open file `{path:#?}`: {error}")))?
            .read_to_end(&mut buffer)
            .await
            .map_err(|error|
                crate::error::error(format!("Unable to read file `{path:#?}`: {error}")))?;

        let storage: AutoPackageWebStorage = {
            serde_json::from_slice(buffer.as_slice())
                .map_err(|error|
                    crate::error::error(format!("Unable to deserialize storage object: {error}")))?
        };

        let _ = self.load_web_storage(package).await?;

        self.storages.write().await.insert(package.to_string(), Arc::new(RwLock::new(storage)));

        Ok(())
    }

    async fn clear(&self, package: &str) -> XReason {
        let storage = self.load_web_storage(package).await?;
        let mut storage = storage.write().await;
        storage.package.clear();
        storage.lections.clear();

        Ok(())
    }

    async fn delete(&self, package: &str, lection: Option<&str>, key: &str) -> XResult<Option<String>> {
        self.ensure_existence(package, lection).await?;

        match lection {
            Some(lection) => {
                let origin = {
                    self.packages
                        .read()
                        .await
                        .get(package)
                        .expect("Ensure existence is broken")
                        .raw_lection(lection)
                        .expect("Ensure existence is broken")
                        .origin()
                        .to_string()
                };

                Ok(
                    self.load_web_storage(package)
                        .await?
                        .write()
                        .await
                        .lections
                        .entry(origin)
                        .or_default()
                        .remove(key)
                )
            }
            None => Ok(
                self.load_web_storage(package)
                    .await?
                    .write()
                    .await
                    .package
                    .remove(key)
            )
        }
    }

    async fn get(&self, package: &str, lection: Option<&str>, key: &str, fallback: Option<&str>) -> XResult<Option<String>> {
        self.ensure_existence(package, lection).await?;

        match lection {
            Some(lection) => {
                let origin = {
                    self.packages
                        .read()
                        .await
                        .get(package)
                        .expect("Ensure existence is broken")
                        .raw_lection(lection)
                        .expect("Ensure existence is broken")
                        .origin()
                        .to_string()
                };

                Ok(
                    self.load_web_storage(package)
                        .await?
                        .write()
                        .await
                        .lections
                        .entry(origin)
                        .or_default()
                        .get(key)
                        .cloned()
                        .or(fallback.map(ToString::to_string))
                )
            }
            None => Ok(
                self.load_web_storage(package)
                    .await?
                    .write()
                    .await
                    .package
                    .get(key)
                    .cloned()
                    .or(fallback.map(ToString::to_string))
            )
        }
    }

    async fn set(&self, package: &str, lection: Option<&str>, key: &str, value: &str, replaced: bool) -> XResult<Option<String>> {
        self.ensure_existence(package, lection).await?;

        match lection {
            Some(lection) => {
                let mut previous = None;

                let origin = {
                    self.packages
                        .read()
                        .await
                        .get(package)
                        .expect("Ensure existence is broken")
                        .raw_lection(lection)
                        .expect("Ensure existence is broken")
                        .origin()
                        .to_string()
                };

                self.load_web_storage(package)
                    .await?
                    .write()
                    .await
                    .lections
                    .entry(origin)
                    .or_default()
                    .entry(key.to_string())
                    .and_modify(|stored| if replaced {
                        previous.replace(std::mem::replace(stored, value.to_string()));
                    })
                    .or_insert(value.to_string());
                
                Ok(previous)
            },
            None => {
                let mut previous = None;

                self.load_web_storage(package)
                    .await?
                    .write()
                    .await                
                    .package
                    .entry(key.to_string())
                    .and_modify(|stored| if replaced {
                        previous.replace(std::mem::replace(stored, value.to_string()));
                    })
                    .or_insert(value.to_string());
                
                Ok(previous)
            },
        }
    }
}
