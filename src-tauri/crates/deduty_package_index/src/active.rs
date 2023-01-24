use std::collections::HashMap;
use std::sync::Arc;

use async_std::sync::RwLock;

use deduty_package_traits::DedutyPackage;

// Arc + RwLock - because can be used in several places at the same time
// Option - because package can be sub while using by other

pub enum ActivePackage {
    Online(Box<dyn DedutyPackage>),
    Offline
}

pub type ActiveItem = Arc<RwLock<ActivePackage>>;

pub struct ActiveStorage {
    packages: RwLock<HashMap<String, ActiveItem>>
}

impl ActiveStorage {
    pub fn new() -> Self {
        Self { packages: RwLock::new(HashMap::new()) }
    }

    pub async fn add(&self, id: String, package: Box<dyn DedutyPackage>) -> Option<ActiveItem> { 
        self.packages
            .write()
            .await
            .insert(id, Arc::new(RwLock::new(ActivePackage::Online(package))))
    }

    pub async fn get(&self, id: &String) -> Option<ActiveItem> {
        self.packages
            .read()
            .await
            .get(id)
            .cloned()
    }

    pub async fn list(&self) -> Vec<String> {
        self.packages
            .read()
            .await
            .keys()
            .cloned()
            .collect()
    }

    pub async fn sub(&self, id: &String) -> Option<ActiveItem> {
        self.packages
            .write()
            .await
            .remove(id)
    }
}
