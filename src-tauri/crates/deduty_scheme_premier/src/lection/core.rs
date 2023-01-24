use async_std::path::{Path, PathBuf};
use uuid::Uuid;

use deduty_package_traits::{
    DedutyFileCollection,
    DedutyLection,
    DedutyLectionMeta
};
use xresult::{ XError, XResult };

use crate::file::{
    PremierFile,
    PremierFileAlias
};
use super::PremierLectionFileCollection;
use super::meta::PremierLectionMeta;


pub struct PremierLection {
    id: Uuid,
    meta: PremierLectionMeta,
    files: PremierLectionFileCollection
}

impl PremierLection {
    pub async fn from(schema: crate::schemes::lection::PremierLection, root: &Path) -> XResult<Self> {
        let mut files = Vec::new();

        if let Some(ref pages) = schema.lection.pages {
            for page in pages {
                let path = root.join(PathBuf::from(&page.relative));
                if !path.exists().await {
                    let location = path.as_os_str().to_string_lossy();
                    return Err(Box::new(XError::from(("PremierLectionError", format!("Lection page doesn't exist at `{}`", location)))));
                }
                if !path.is_file().await {
                    let location = path.as_os_str().to_string_lossy();
                    return Err(Box::new(XError::from(("PremierLectionError", format!("{} is not a lection page", location)))));
                }
                files.push(PremierFile::new(PremierFileAlias::NoAlias, root.to_path_buf(), path, uuid::Uuid::new_v4()));
            }
        }

        Ok(
            Self {
                id: uuid::Uuid::new_v4(),
                meta: schema.lection.into(),
                files: PremierLectionFileCollection::from(files)
            }
        )
    }
}

impl DedutyLection for PremierLection {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn meta(&self) -> &dyn DedutyLectionMeta {
        &self.meta
    }

    fn files(&self) -> &dyn DedutyFileCollection {
        &self.files
    }
}

impl From<PremierLection> for Box<dyn DedutyLection> {
    fn from(lection: PremierLection) -> Self {
        Box::new(lection)
    }
}
