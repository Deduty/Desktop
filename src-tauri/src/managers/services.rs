use std::collections::HashMap;
use std::sync::Arc;

use async_std::sync::RwLock;
use xresult::{ XResult, XError, XReason };

use deduty_package::{ DedutyPackage, DedutyLection, DedutyFile };
use deduty_service::{ Service, IndexService, WebStorageService };

use crate::utils::{ Borrowed, BorrowedItem };



pub struct ServiceEntry {
    service: Option<Arc<dyn Service>>,
    service_id: String
}

impl ServiceEntry {
    pub fn as_service(&self) -> XResult<Borrowed<dyn Service>> {
        match self.service.clone() {
            Some(service) => Ok(BorrowedItem::boxed(service)),
            None => XError::from(("Service error", format!("Service with id `{}` is not exist", self.service_id))).into()
        }
    }

    pub fn as_web_storage(&self) -> XResult<Borrowed<dyn WebStorageService>> {
        match self.service.clone() {
            Some(service) => Ok(BorrowedItem::boxed(service as Arc<dyn WebStorageService>)),
            None => XError::from(("Service error", format!("Service with id `{}` is not exist", self.service_id))).into()
        }
    }

    pub async fn with_package(&self, package: &str) -> XResult<Borrowed<dyn DedutyPackage>> {
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

    pub async fn with_lection(&self, package: &str, lection: &str) -> XResult<Borrowed<dyn DedutyLection>> {
        match self.with_package(package).await?.borrow().lection(lection).await? {
            Some(lection) => Ok(lection),
            None => XError::from(("Service error", format!("Lection with id `{lection}` not found at package with id `{package}` not found at `{}`", self.service_id))).into()
        }
    }

    pub async fn with_file(&self, package: &str, lection: &str, file: &str) -> XResult<Borrowed<dyn DedutyFile>> {
        match self.with_lection(package, lection).await?.borrow().file(file).await? {
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

    pub async fn list(&self) -> impl Iterator<Item = Borrowed<dyn Service>> {
        self.services
            .read()
            .await
            .values()
            .cloned()
            .map(BorrowedItem::boxed)
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

    pub async fn get(&self, service: &str) -> Option<Borrowed<dyn Service>> {
        self.services
            .read()
            .await
            .get(service)
            .cloned()
            .map(BorrowedItem::boxed)
    }

    pub async fn sub(&self, service: &str) -> Option<Borrowed<dyn Service>> {
        self.services
            .write()
            .await
            .remove(service)
            .map(BorrowedItem::boxed)
    }
}
