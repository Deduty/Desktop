use async_std::{
    path::PathBuf,
    sync::Arc
};
use async_trait::async_trait;
use deduty_package::{ DedutyPackage, UniquePackage };
use deduty_service::AddService;
use xresult::XResult;

use crate::package::AutoPackage;
use crate::borrowed::{ Borrowed, BorrowedItem };

use super::{ AutoPackageService, Requirement };


#[async_trait]
impl AddService for AutoPackageService {
    fn requirements(&self) -> &str {
        r#"{ "path": "DirectoryPath" }"#
    }

    async fn add(&self, serialized: &str) -> XResult<Borrowed<dyn DedutyPackage>> {
        let path = serde_json::from_str::<Requirement>(serialized)
            .map_err(|error|
                crate::error::error(format!("Unable deserialize add requirements: {error}")))?
            .path;
        
        let package = Arc::new(AutoPackage::load(PathBuf::from(path)).await?);
        let mut packages = self.packages.write().await;

        if packages.contains_key(package.id()) {
            return crate::error::error_err(format!("Package with same uuid already exist: {}", package.id()));
        }

        packages.insert(package.id().to_string(), package.clone());
        Ok(BorrowedItem::boxed(package as Arc<dyn DedutyPackage>))
    }
}
