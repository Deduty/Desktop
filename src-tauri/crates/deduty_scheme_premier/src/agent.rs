use deduty_package_traits::DedutyPackage;
use deduty_package_index::{
    PackageAgent,
    PackageStatus
};

use xresult::{ XError, XReason };


pub struct PremierPackageAgent {
    package: Option<Box<dyn DedutyPackage>>,
    online: bool
}

impl PremierPackageAgent {
    pub fn new(package: Box<dyn DedutyPackage>) -> Self {
        Self { package: Some(package), online: true }
    }
}

impl PackageAgent for PremierPackageAgent {
    fn online(&mut self) -> XReason {
        match self.package {
            Some(_) => Ok(self.online = true),
            None => Err(Box::new(XError::from(("Package agent error", "Package was removed"))))
        }
    }

    fn offline(&mut self) -> XReason {
        Ok(self.online = false)
    }

    fn package_ref(&self) -> PackageStatus<&dyn DedutyPackage> {
        match (self.online, self.package.as_ref()) {
            (true, Some(package)) => PackageStatus::Online(package.as_ref()),
            _ => PackageStatus::Offline
        }
    }

    fn package_mut(&mut self) -> PackageStatus<&mut dyn DedutyPackage> {
        match (self.online, self.package.as_mut()) {
            (true, Some(package)) => PackageStatus::Online(package.as_mut()),
            _ => PackageStatus::Offline
        }
    }
}
