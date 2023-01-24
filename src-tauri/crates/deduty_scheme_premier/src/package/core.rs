use std::collections::HashMap;

use async_std::io::ReadExt;
use async_std::fs::File;
use async_std::path::{ Path, PathBuf };
use async_std::stream::StreamExt;
use regex::Regex;
use uuid::Uuid;

use deduty_package_traits::{
    DedutyFileCollection,
    DedutyLection,
    DedutyPackageMeta,
    DedutyPackage,
};
use xresult::{ XError, XResult };

use crate::schemes;
use crate::file::{
    PremierFile,
    PremierFileAlias
};
use crate::lection::PremierLection;

use super::collection::PremierPackageFileCollection;
use super::meta::PremierPackageMeta;

pub struct PremierPackage {
    id: Uuid,
    meta: PremierPackageMeta,
    files: PremierPackageFileCollection,
    lections: Vec<Box<dyn DedutyLection>>
}

impl PremierPackage {
    pub async fn from(schema: schemes::package::PremierPackage, root: &Path) -> XResult<Self> {
        let mut files = HashMap::new();

        // About file: test, include
        {
            let about = PremierFile::new(
                PremierFileAlias::Alias("about".into()),
                root.to_path_buf(),
                PathBuf::from(&schema.package.about.clone().unwrap_or("ABOUT.md".into())),
                Uuid::new_v4()
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

        // Lection candidates collection
        let mut candidates: Vec<PathBuf> = vec![];

        // EXACT
        if let Some(lections) = schema.lections.exact {
            for lection in lections {
                let mut lection_root = root.join(lection.relative);
                if !lection_root.exists().await {
                    let location = lection_root.as_os_str().to_string_lossy();
                    return Err(Box::new(XError::from(("PremierPackageError", format!("Lection doesn't exist at {}", location)))));
                }

                match lection_root.is_file().await {
                    // Exact is a lection.toml path
                    true => {
                        if !lection_root.ends_with("lection.toml") {
                            let location = lection_root.as_os_str().to_string_lossy();
                            return Err(Box::new(XError::from(("PremierPackageError", format!("Location `{}` is not a `lection.toml` file", location)))));
                        }
                        candidates.push(lection_root);
                    }
                    // Exact is a folder that contains lection.toml path
                    false => {
                        lection_root = lection_root.join("lection.toml");
                        if !lection_root.exists().await {
                            let location = lection_root.as_os_str().to_string_lossy();
                            return Err(Box::new(XError::from(("PremierPackageError", format!("Location doesn't exist at `{}`", location)))));
                        }
                        if !lection_root.ends_with("lection.toml") {
                            let location = lection_root.as_os_str().to_string_lossy();
                            return Err(Box::new(XError::from(("PremierPackageError", format!("Location `{}` is not a `lection.toml` file", location)))));
                        }
                        candidates.push(lection_root);
                    }
                }
            }
        }

        // REGEX
        if let Some(expression) = schema.lections.regex {
            let regex = Regex::new(&expression)
                .map_err(|error| Box::new(XError::from(("PremierPackageError", error.to_string()))))?;

            let mut folders = vec![root.to_path_buf()];
            while !folders.is_empty() {
                // UNWRAP: Directory already non empty
                let folder = folders.pop().unwrap();

                let mut content = async_std::fs::read_dir(&folder).await?;
                while let Some(target) = content.next().await {
                    let path = target
                        .map_err(|error| Box::new(XError::from(("PremierPackageError", error.to_string()))))?
                        .path();

                    if path.is_dir().await {
                        folders.push(path.clone())
                    }

                    // SLASH NORMALIZATION
                    let repr = path.to_string_lossy().to_string().replace("\\", "/");
                    if regex.is_match(&repr) {
                        candidates.push(path);
                    }
                }
            }
        }

        // Exact Lections
        let mut lections: Vec<Box<dyn DedutyLection>> = vec![];

        for candidate in candidates {
            let lection_toml = candidate.clone();
            let lection_root = candidate
                .parent()
                .ok_or_else(|| Box::new(XError::from(("PremierPackageError", format!("Lection path have no parents at {}", candidate.as_os_str().to_string_lossy())))))?;

            // CHECKS MUST BE PERFORMED BEFORE
    
            let mut buffer = Vec::new();
            File::open(lection_toml.as_path())
                .await
                .map_err(|error| error.to_string())?
                .read_to_end(&mut buffer)
                .await
                .map_err(|error| error.to_string())?;
    
            let lection = toml::from_slice::<crate::schemes::lection::PremierLection>(&buffer)?;
            lections.push(Box::new(PremierLection::from(lection, &lection_root).await?));
        }

        lections.sort_by_key(|lection| lection.meta().order());

        Ok(
            PremierPackage {
                id: Uuid::new_v4(),
                meta: schema.package.into(),
                files: PremierPackageFileCollection::from(files),
                lections
            }
        )
    }
}

impl DedutyPackage for PremierPackage {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn files(&self) -> &dyn DedutyFileCollection {
        &self.files
    }

    fn meta(&self) -> &dyn DedutyPackageMeta {
        &self.meta
    }

    fn lections(&self) -> Vec<&dyn DedutyLection> {
        self.lections
            .iter()
            .map(|lection| lection.as_ref())
            .collect()
    }
}

impl From<PremierPackage> for Box<dyn DedutyPackage> {
    fn from(package: PremierPackage) -> Self {
        Box::new(package)
    }
}
