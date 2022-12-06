use std::collections::HashMap;

use async_std::path::{ Path, PathBuf };
use uuid::Uuid;

use package::file::traits::{
    DedutyFileCollection,
    DedutyFile
};
use package::package::traits::{
    DedutyPackageMeta,
    DedutyPackage,
};

use crate::schemes;

use super::collection::PremierPackageFileCollection;
use super::meta::PremierPackageMeta;

pub struct PremierPackage {
    id: Uuid,
    meta: PremierPackageMeta,
    files: PremierPackageFileCollection
}

impl PremierPackage {
    pub async fn from(schema: schemes::package::PremierPackage, root: &Path) -> Self {
        let mut files = HashMap::new();

        // About file: test, include
        {
            let about = super::file::PremierFile::new(
                super::file::PremierFileAlias::Alias("about".into()),
                root.to_path_buf(),
                PathBuf::from(&schema.package.about.clone().unwrap_or("ABOUT.md".into())),
            );

            match about.location().await {
                Ok(location) => {
                    if location.exists().await {
                        files.insert("about".to_string(), about);
                    }
                },
                Err(_) => {}
            }
        }

        PremierPackage {
            id: Uuid::new_v4(),
            meta: schema.package.into(),
            files: PremierPackageFileCollection::from(files),
        }
    }
}

impl DedutyPackage for PremierPackage {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn files(&self) -> &dyn DedutyFileCollection {
        &self.files
    }

    fn meta(&self) -> &dyn DedutyPackageMeta {
        &self.meta
    }
}

impl From<PremierPackage> for Box<dyn DedutyPackage> {
    fn from(package: PremierPackage) -> Self {
        Box::new(package)
    }
}
