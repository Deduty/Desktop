use std::collections::HashMap;
use std::sync::Arc;

use async_std::path::{ Path, PathBuf };
use async_std::stream::StreamExt;
use async_std::sync::RwLock;
use async_trait::async_trait;

use deduty_package_traits::DedutyPackage;
use deduty_package_index::{
    IndexService,
    PackageAgent,
    PackageStatus
};

use xresult::{ XError, XReason, XResult };

use crate::package::PremierPackage;
use crate::agent::PremierPackageAgent;

type SafePackageAgent = Arc<RwLock<Box<dyn PackageAgent>>>;


pub struct PremierIndexService {
    packages: HashMap<String, SafePackageAgent>,
    root: PathBuf
}

impl PremierIndexService {
    pub fn new(root: PathBuf) -> Self {
        Self { packages: HashMap::new(), root }
    }
}

#[async_trait]
impl IndexService for PremierIndexService {
    async fn load_all(&mut self) -> XResult<Vec<XReason>> {
        let mut reasons = Vec::new();

        let mut targets = self.root
            .read_dir()
            .await
            .map_err(|error| Box::new(XError::from(("Premier index service error", error.to_string()))))?
            .enumerate();
        
        while let Some((_, target)) = targets.next().await {
            match target {
                Ok(entry) => {
                    match PremierPackage::load(entry.path()).await {
                        Ok(package) => {
                            reasons.push(Ok(()));
                            let package_id = package.id();
                            let agent = Box::new(PremierPackageAgent::new(Box::new(package) as Box<dyn DedutyPackage>)) as Box<dyn PackageAgent>;
                            self.packages.insert(package_id, Arc::new(RwLock::new(agent)));
                        }
                        Err(error) => reasons.push(Err(error))
                    }
                }
                Err(error) => reasons.push(XError::from(("Premier index service error", error.to_string())).into())
            }
        }

        Ok(reasons)
    }

    async fn save_all(&mut self) -> XResult<Vec<XReason>> {
        let mut reasons = Vec::with_capacity(self.packages.len());
        let mut failures = Vec::with_capacity(self.packages.len());

        for (id, agent) in self.packages.iter() {
            match agent.write().await.package_mut() {
                PackageStatus::Online(package) => {
                    match package.as_any_mut().downcast_mut::<PremierPackage>() {
                        Some(premier) => {
                            let expected_root = self.root.join(Path::new(premier.id().as_str()));

                            match premier.root == expected_root {
                                true => reasons.push(Ok(())),
                                false => {
                                    let result = premier.save(&expected_root).await;
                                    if result.is_err() {
                                        failures.push(premier.id());
                                    }
                                    reasons.push(result);
                                }
                            }
                        }
                        None => reasons.push(XError::from((
                            "Premier index service fatal error",
                            format!("Premier services owns wrong package {} with unknown type. This must be impossible", id)
                        )).into())
                    }
                }
                PackageStatus::Offline => reasons.push(XError::from((
                    "Premier index service error",
                    format!("Unable to save package with id {}", id)
                )).into())
            }
        }

        for failure in failures {
            self.packages
                .get_mut(&failure)
                .expect("Internal mutability error. Failure id, that was cloned from hashmap, doesn't contains by this hashmap")
                .write()
                .await
                .offline()
                .expect("Unable to disable failure package. This error can lead to further errors - shutdown");
        }

        Ok(reasons)
    }

    async fn add(&mut self, serialized: String) -> XResult<SafePackageAgent> {
        let package = PremierPackage::load(PathBuf::from(serialized)).await?;
        let package_id = package.id();
        let agent = Box::new(PremierPackageAgent::new(Box::new(package) as Box<dyn DedutyPackage>)) as Box<dyn PackageAgent>;
        let safe_agent = Arc::new(RwLock::new(agent));
        self.packages.insert(package_id, safe_agent.clone());
        Ok(safe_agent)
    }

    async fn get(&self, id: &String) -> XResult<Option<SafePackageAgent>> {
        Ok(self.packages.get(id).cloned())
    }

    async fn has(&self, id: &String) -> XResult<bool> {
        Ok(self.packages.contains_key(id))
    }

    async fn list(&self) -> XResult<Vec<String>> {
        Ok(self.packages.keys().cloned().collect())
    }

    async fn sub(&mut self, id: &String) -> XReason {
        match self.packages.remove(id) {
            Some(package) => {
                // Remove package if it's path under application control
                let package_root = XResult::from(package.read().await.package_ref())?
                    .as_any_ref()
                    .downcast_ref::<PremierPackage>()
                    .ok_or_else(|| Box::new(XError::from(("Premier index service error", format!("Deduty package {id} has wrong type. Unable to clean directory")))))?
                    .root
                    .clone();

                if package_root.starts_with(&self.root) {
                    return async_std::fs::remove_dir_all(package_root)
                        .await
                        .map_err(|error| XError::from(("Premier index service error", format!("Unable to remove package {id} due to unexpected error: {error}"))).into());
                }

                // TODO: What to do if this error occurs?
                package
                    .write()
                    .await
                    .offline()
                    .map_err(|error| XError::from(("Premier index service error", format!("Unable offline package with id {id}. Unexpected error: {error}"))).into())
            }
            None => XError::from(("Premier index service error", format!("Package with id {} not found", id))).into()
        }
    }

    async fn update(&mut self, _serialized: String, _id: &String) -> XReason {
        // TODO: Implement it later
        XError::from(("Premier index service error", "Update function is not implemented")).into()
    }
}
