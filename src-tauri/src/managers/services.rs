use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

use async_std::sync::RwLock;
use xresult::{ XResult, XError, XReason };

use deduty_package::{ DedutyPackage, DedutyLection, DedutyFile };
use deduty_service::{ Service, IndexService, WebStorageService };


type ServiceRef = Box<dyn Deref<Target = dyn Service> + Send + Sync>;

fn into_service_ref(service: Arc<dyn Service>) -> ServiceRef {
    Box::new(service) as ServiceRef
}


pub struct ServiceEntry {
    service: Option<Arc<dyn Service>>,
    service_id: String
}

impl ServiceEntry {
    pub fn as_service(&self) -> XResult<&dyn Service> {
        match &self.service {
            Some(service) => Ok(service.as_ref()),
            None => XError::from(("Service error", format!("Service with id `{}` is not exist", self.service_id))).into()
        }
    }

    pub fn as_web_storage(&self) -> XResult<&dyn WebStorageService> {
        Ok(self.as_service()? as &dyn WebStorageService)
    }

    pub async fn with_package(&self, package: &str) -> XResult<&dyn DedutyPackage> {
        match &self.service {
            Some(service) => {
                match (service.as_ref() as &dyn IndexService).get(package).await? {
                    Some(package) => Ok(package),
                    None => XError::from(("Service error", format!("Package with id `{package}` not found at `{}`", self.service_id))).into()
                }
            },
            None => XError::from(("Service error", format!("Service with id `{}` is not exist", self.service_id))).into()
        }
    }

    pub async fn with_lection(&self, package: &str, lection: &str) -> XResult<&dyn DedutyLection> {
        match self.with_package(package).await?.lection(lection).await? {
            Some(lection) => Ok(lection),
            None => XError::from(("Service error", format!("Lection with id `{lection}` not found at package with id `{package}` not found at `{}`", self.service_id))).into()
        }
    }

    pub async fn with_file(&self, package: &str, lection: &str, file: &str) -> XResult<&dyn DedutyFile> {
        match self.with_lection(package, lection).await?.file(file).await? {
            Some(file) => Ok(file),
            None => XError::from(("Service error", format!("File with id `{file}` not found at lection with id `{lection}` not found at package with id `{package}` not found at `{}`", self.service_id))).into()
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

    pub async fn access(&self, service_id: &str) -> ServiceEntry {
        let service =
            self.services
                .read()
                .await
                .get(service_id)
                .cloned();

        ServiceEntry { service, service_id: service_id.to_string() }
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
            .insert(service.id().to_string(), service.into());
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
