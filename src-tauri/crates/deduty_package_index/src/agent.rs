use deduty_package_traits::DedutyPackage;

use xresult::{ XError, XReason, XResult };


pub enum PackageStatus<T> {
    Online(T),
    Offline
}

impl<T> PackageStatus<T> {
    pub fn to_option(self) -> Option<T> {
        match self {
            PackageStatus::Online(package) => Some(package),
            PackageStatus::Offline => None,
        }
    }

    pub fn to_result(self) -> XResult<T> {
        match self {
            PackageStatus::Online(package) => Ok(package),
            PackageStatus::Offline => Err(Box::new(XError::from(("Deduty package index error", "Package is not available"))))
        }
    }
}


pub trait PackageAgent: Send + Sync {
    fn online(&mut self) -> XReason;
    fn offline(&mut self) -> XReason;

    fn package_ref(&self) -> PackageStatus<&dyn DedutyPackage>;
    fn package_mut(&mut self) -> PackageStatus<&mut dyn DedutyPackage>;
}
