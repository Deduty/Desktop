pub mod serde;
pub mod traits;

impl From<&dyn traits::DedutyPackageMeta> for serde::DedutyPackageMeta {
    fn from(object: &dyn traits::DedutyPackageMeta) -> Self {
        Self {
            name: object.name().clone(),
            version: object.version().clone(),
            language: object.language().clone(),
            tags: object.tags().clone()
        }
    }
}

impl From<&dyn traits::DedutyPackage> for serde::DedutyPackage {
    fn from(object: &dyn traits::DedutyPackage) -> Self {
        Self {
            id: object.id().to_string(),
            meta: object.meta().into(),
            files: object.files().into(),
        }
    }
}
