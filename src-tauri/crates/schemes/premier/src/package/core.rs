use async_std::path::{Path, PathBuf};
use uuid::Uuid;

use package::file::traits::DedutyFileCollection;
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
    pub fn from(schema: schemes::package::PremierPackage, root: &Path) -> Self {
        let path = schema.package.about.clone().unwrap_or("./ABOUT.md".into());
        let about = super::file::PremierFile::new(
            super::file::PremierFileAlias::Alias("about".into()),
            root.to_path_buf(),
            PathBuf::from(&path)
        );
        let files = PremierPackageFileCollection::from([("about".into(), about)].into());

        let meta = schema.package.into();
        PremierPackage { id: Uuid::new_v4(), meta, files }
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
