use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

use async_std::sync::RwLock;
use xresult::{ XResult, XError, XReason };

use deduty_package::{ DedutyPackage, DedutyLection, DedutyFile };
use deduty_service::Service;


type ServiceRef = Box<dyn Deref<Target = dyn Service> + Send + Sync>;

fn into_service_ref(service: Arc<dyn Service>) -> ServiceRef {
    Box::new(service) as ServiceRef
}


enum ServiceEntry {
    Some(ServiceRef),
    None
}

impl ServiceEntry {
    pub async fn with_package(&self, package: &str) -> XResult<Option<&dyn DedutyPackage>> {
        match self {
            Self::Some(service) => {
                service.get(package).await
            },
            Self::None => Ok(None)
        }
    }

    pub async fn with_lection(&self, package: &str, lection: &str) -> XResult<Option<&dyn DedutyLection>> {
        match self {
            Self::Some(service) => {
                Ok(
                    service
                        .get(package)
                        .await?
                        .ok_or_else(|| XError::from(("Service error", format!("Package with id `{package}` not found at `{}`", service.id()))))?
                        .lection(lection)
                )
            },
            Self::None => Ok(None)
        }
    }

    pub async fn with_file(&self, package: &str, lection: &str, file: &str) -> XResult<Option<&dyn DedutyFile>> {
        match self {
            Self::Some(service) => {
                Ok(
                    service
                        .get(package)
                        .await?
                        .ok_or_else(|| XError::from(("Service error", format!("Package with id `{package}` not found at `{}`", service.id()))))?
                        .lection(lection)
                        .ok_or_else(|| XError::from(("Service error", format!("Lection with id `{lection}` not found at package with id `{package}` not found at `{}`", service.id()))))?
                        .file(file)
                        .await?
                )
            },
            Self::None => Ok(None)
        }
    }
}


#[derive(Default)]
pub struct ServiceManager {
    services: RwLock<HashMap<String, Arc<dyn Service>>>
}

impl ServiceManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn access(&self, service: &str) -> ServiceEntry {
        match self.services.read().await.get(service) {
            Some(service) => ServiceEntry::Some(Box::new(service.clone())),
            None => ServiceEntry::None
        }
    }

    pub async fn list(&self) -> impl Iterator<Item = ServiceRef> {
        self.services
            .read()
            .await
            .values()
            .cloned()
            .map(into_service_ref)
            .collect::<Vec<_>>().into_iter()
    }

    pub async fn add(&self, service: Box<dyn Service>) -> XReason {
        if self.services.read().await.contains_key(service.id()) {
            return XError::from(("Service error", format!("Unable to add a new service with an existed id `{}`", service.id()))).into();
        }

        self.services
            .write()
            .await
            .insert(service.id().clone(), service.into());
        Ok(())
    }

    pub async fn has(&self, service: &str) -> bool {
        self.services
            .read()
            .await
            .contains_key(service)
    }

    pub async fn get(&self, service: &str) -> Option<ServiceRef> {
        self.services
            .read()
            .await
            .get(service)
            .cloned()
            .map(into_service_ref)
    }

    pub async fn sub(&self, service: &str) -> Option<ServiceRef> {
        self.services
            .write()
            .await
            .remove(service)
            .map(into_service_ref)
    }
}
