use std::collections::HashMap;

use async_std::{
    fs::File,
    io::{ReadExt, WriteExt},
    path::PathBuf,
    stream::StreamExt,
    sync::Arc
};
use deduty_package::{ MetaLection, UniqueLection };
use serde::{ Deserialize, Serialize };
use xresult::{ XError, XReason, XResult };

use crate::lection::AutoLection;
use crate::schemes::{ Package, PackageMeta, PackageLections, PackageManifest };

mod meta;
mod peek;
mod read;
mod unique;


fn os_to_string(os: &std::ffi::OsStr) -> Option<String> {
    os.to_str().map(ToString::to_string)
}


#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AutoPackageMeta {
    pub name: String,
    pub version: Option<String>,
    pub language: Option<String>,
    pub tags: Option<Vec<String>>
}


#[derive(Debug, Default)]
pub struct AutoPackage {
    id: String,
    size: usize,
    path: PathBuf,

    about: String,

    serde_meta: AutoPackageMeta,
    meta: Option<String>,

    lections_order: Vec<Arc<AutoLection>>,
    lections_index: HashMap<String, Arc<AutoLection>>
}

impl AutoPackage {
    pub async fn load(path: PathBuf) -> XResult<Self> {
        if !path.is_dir().await {
            return crate::error::error_err(format!("Path `{path:#?}` is not a directory"));
        }

        let mut about = "about".to_string();

        let mut id = uuid::Uuid::new_v4().to_string();

        let mut name = id.to_string();

        let package: Option<Package> = {
            let package_toml = path.join("package.toml");

            let mut buffer = Vec::new();
            File::open(&package_toml)
                .await
                .map_err(|error|
                    crate::error::error(format!("Unable to open file `{package_toml:#?}`: {error}")))?
                .read_to_end(&mut buffer)
                .await
                .map_err(|error|
                    crate::error::error(format!("Unable to read file `{package_toml:#?}`: {error}")))?;
    
            let package_meta: Package = toml::from_slice(&buffer)
                .map_err(|error|
                    crate::error::error(format!("Unable to get package meta from `{package_toml:#?}`: {error}")))?;

            if package_meta.manifest.name.eq_ignore_ascii_case("auto") {
                return crate::error::error_err(format!("Manifest name `{}` is not supported", package_meta.manifest.name));
            }

            Some(package_meta)
        };

        let mut lections = Vec::new();
        {
            let mut entries = async_std::fs::read_dir(&path)
                .await
                .map_err(|error| crate::error::error(format!("Unable to scan directory {path:#?}: {error}")))?;

            while let Some(entry) = entries.next().await {
                let Ok(path) = entry.map(|entry| entry.path()) else {
                    continue;
                };

                let Some(name) = path.file_name().and_then(os_to_string) else {
                    continue;
                };

                if !path.is_dir().await {
                    continue;
                }

                let lection = Arc::new(AutoLection::load(path, name.eq_ignore_ascii_case(&about)).await?);
                lections.push((name, lection));
            }
        }

        let (meta_package, meta_lection) = {
            package
                .map(|meta| (meta.package, meta.lection))
                .unwrap_or((None, None))
        };

        {
            let mut first_names = HashMap::new();
            let mut last_names = HashMap::new();

            if let Some(lection) = meta_lection {
                if let Some(first) = lection.first {
                    for (order, name) in first.into_iter().enumerate() {
                        first_names.insert(name, order);
                    }
                }

                if let Some(last) = lection.last {
                    for (order, name) in last.into_iter().enumerate() {
                        last_names.insert(name, order);
                    }
                }
            }

            lections.sort_by(|(lhs, _), (rhs, _)| {
                match (first_names.get(lhs), last_names.get(lhs), first_names.get(rhs), last_names.get(rhs)) {
                    // <l, >l, <r, >r

                    // <l, <r
                    // Both first -> compare positions in first array
                    (Some(lhs_first), None, Some(rhs_first), None) => lhs_first.cmp(rhs_first),

                    // >l, >r
                    // Both last -> compare positions in last array
                    (None, Some(lhs_last), None, Some(rhs_last)) => lhs_last.cmp(rhs_last),

                    // <l, >r
                    // Lhs in first, rhs in last
                    (Some(_), None, None, Some(_)) => std::cmp::Ordering::Less,

                    // >l, <r
                    // Lhs in last, rhs in first
                    (None, Some(_), Some(_), None) => std::cmp::Ordering::Greater,

                    // Any strange case and no order case -> cmp by name
                    _ => lhs.cmp(rhs)
                }
            });
        }

        let package_meta: Option<AutoPackageMeta> = match meta_package {
            Some(meta) => {
                id = meta.id.clone().unwrap_or(id);
                about = meta.about.clone().unwrap_or(about);
                name = meta.name.clone().unwrap_or(name);

                Some(
                    AutoPackageMeta {
                        name: name.to_string(),
                        language: meta.language,
                        version: meta.version,
                        tags: meta.tags
                    }
                )
            }
            None => None,
        };

        let mut size = 0usize;
        for (_, lection) in lections.iter() {
            size += lection.size().expect("Auto service must know file size");
        }

        let meta = match package_meta.as_ref() {
            None => None,
            Some(meta) => Some(
                serde_json::to_string(&meta)
                    .map_err(|error| crate::error::error(format!("Unable to serialize meta for `{path:#?}`: {error}")))?
            )
        };

        Ok(
            AutoPackage {
                id,
                size,
                path,

                about,

                meta,
                serde_meta: package_meta.unwrap_or_default(),

                lections_index: lections.iter().map(|(_, lection)| (lection.id().to_string(), lection.clone())).collect(),
                lections_order: lections.into_iter().map(|(_, lection)| lection).collect()
            }
        )
    }

    /// Removes only files that locates under root
    /// Makes it recursively
    ///
    /// Note: This method must be called after package removing from index
    ///       otherwise it can lead errors
    pub async fn remove(&self, root: &PathBuf) -> XReason {
        if !self.path.starts_with(root) {
            return Ok(());
        }

        async_std::fs::remove_dir_all(self.path.clone())
            .await
            .map_err(|error|
            crate::error::error(format!("Unable to remove directory `{:#?}`: {error}", self.path)))
    }

    pub async fn save(&self, root: &PathBuf) -> XReason {
        if self.path.starts_with(root) {
            return Ok(())
        }

        let package_root = root.join(&self.id);

        // Non critical since path may exist
        let _ = async_std::fs::create_dir(&package_root).await;

        // Create package.toml file
        let package_toml_content = Package {
            manifest: PackageManifest { name: "Auto".to_string() },
            package: Some(PackageMeta {
                id: Some(self.id.to_string()),
                about: Some(self.about.to_string()),
                name: Some(self.serde_meta.name.clone()),
                version: self.serde_meta.version.clone(),
                language: self.serde_meta.language.clone(),
                tags: self.serde_meta.tags.clone(),
            }),
            lection: Some(PackageLections {
                first: Some(self.lections_order.iter().map(|lection| lection.id().to_string()).collect()),
                last: None
            })
        };

        let serialized = toml::to_vec(&package_toml_content)
            .map_err(|error| Box::new(XError::from(("Auto service error", format!("Unable to serialize package metadata for `{}`: {error}", self.id.clone())))))?;
        
        File::create(package_root.join("package.toml"))
            .await
            .map_err(|error|
                crate::error::error(format!("Unable to serialize package meta for `{}`: {error}", self.id)))?
            .write_all(serialized.as_slice())
            .await
            .map_err(|error|
                crate::error::error(format!("Unable to write metadata for `{}`: {error}", self.id)))?;

        for lection in self.lections_order.iter() {
            lection.save(&package_root).await?;
        }

        Ok(())
    }

    /// Sets id according on origin name equality (excluding package itself)
    pub async fn update(&mut self, source: &AutoPackage) {
        self.id = source.id.clone();
    }
}
