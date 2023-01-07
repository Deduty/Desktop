use async_std::path::{Path, PathBuf};
use uuid::Uuid;

use package::file::traits::DedutyFileCollection;
use package::lection::traits::{
    DedutyLection,
    DedutyLectionMeta
};

use crate::error::{ XResult, PremierError };
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
                    return Err(Box::new(PremierError::new(format!("Lection page doesn't exist at {}", path.as_os_str().to_string_lossy()))));
                }
                if !path.is_file().await {
                    return Err(Box::new(PremierError::new(format!("{} is not a lection page", path.as_os_str().to_string_lossy()))));
                }
                files.push(PremierFile::new(PremierFileAlias::NoAlias, root.to_path_buf(), path));
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
    fn id(&self) -> &Uuid {
        &self.id
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
