use std::collections::HashMap;

use async_std::io::{ReadExt, WriteExt};
use async_std::fs::File;
use async_std::path::PathBuf;
use async_std::stream::StreamExt;
use async_trait::async_trait;
use regex::Regex;
use uuid::Uuid;

use deduty_package_traits::{
    DedutyFileCollection,
    DedutyLection,
    DedutyPackageMeta,
    DedutyPackage,
};
use xresult::{ XError, XResult, XReason };

use crate::file::{
    PremierFile,
    PremierFileAlias
};

use super::collection::PremierPackageFileCollection;
use super::meta::PremierPackageMeta;

use crate::lection::PremierLection;
use crate::schemes::package::LectionsExactItem;


pub struct PremierPackage {
    pub(crate) root: PathBuf,
    pub(crate) id: Uuid,
    pub(crate) meta: PremierPackageMeta,
    pub(crate) files: PremierPackageFileCollection,
    pub(crate) lections: Vec<Box<dyn DedutyLection>>
}

impl PremierPackage {
    pub async fn load(path: PathBuf) -> XResult<Self> {
        if !path.exists().await {
            return XError::from(("Premier index service error", format!("Path '{:#?}' is not exist", path))).into();
        }
        if !path.is_dir().await {
            return XError::from(("Premier index service error", format!("Path '{:#?}' is not a directory", path))).into();
        }
    
        let schema = {
            let package_toml = path.join("package.toml");
            if !package_toml.exists().await {
                return Err("'package.toml' is not exist".into());
            }
            if !package_toml.is_file().await {
                return Err("'package.toml' is not a file".into());
            }
    
            let mut buffer = Vec::new();
            File::open(package_toml.as_path())
                .await
                .map_err(|error| error.to_string())?
                .read_to_end(&mut buffer)
                .await
                .map_err(|error| error.to_string())?;
    
            toml::from_slice::<crate::schemes::package::PremierPackage>(&buffer)?
        };

        let mut files = HashMap::new();

        // About file: test, include
        {
            let about = PremierFile::new(
                PremierFileAlias::Alias("about".into()),
                path.to_path_buf(),
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

        // EXACT - Collect lections
        if let Some(lections) = schema.lections.exact {
            for lection in lections {
                let mut lection_root = path.join(lection.relative);
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

        // REGEX - Collect lections
        if let Some(expression) = schema.lections.regex {
            let regex = Regex::new(&expression)
                .map_err(|error| Box::new(XError::from(("PremierPackageError", error.to_string()))))?;

            let mut folders = vec![path.to_path_buf()];
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
                root: path,
                id: schema.package.id
                    .clone()
                    .unwrap_or_else(|| Uuid::new_v4().to_string())
                    .parse::<Uuid>()
                    .map_err(|error| Box::new(XError::from(("PremierPackageError", error.to_string()))))?,
                meta: schema.package.into(),
                files: PremierPackageFileCollection::from(files),
                lections
            }
        )
    }

    pub async fn save(&mut self, path: &PathBuf) -> XReason {
        // Ensure root folder existence
        async_std::fs::create_dir_all(path.clone())
            .await
            .map_err(|error| XError::from(("PremierPackageError", error.to_string())))?;

        // Copy all files if save path is different then current
        if &self.root != path {
            {
                // Copy package files
                // For current implementation, only one file allowed - about
                let mut about = None;

                if let Some(about_file) =
                    self.files
                        .collection
                        .get(&"about".to_string())
                {
                    let about_filename = about_file.path.file_name().expect("Unable to get file name from file");
                    let expected_path = path.join(about_file.path.clone());
                    async_std::fs::copy(about_file.location().await?, expected_path.clone())
                        .await
                        .map_err(|error| XError::from(("PremierPackageError", error.to_string())))?;
                    about = Some(about_filename.to_string_lossy().to_string());
                }

                // Preparing schema
                let lections: Vec<_> =
                    self.lections
                        .iter()
                        .map(|lection|
                            LectionsExactItem {
                                relative: lection.id()
                            }
                        )
                        .collect();

                let package_schema = crate::schemes::package::PremierPackage {
                    manifest: crate::schemes::package::Manifest::default(),
                    lections: crate::schemes::package::LectionsMeta {
                        regex: None,
                        exact: Some(lections)
                    },
                    package: crate::schemes::package::PackageMeta {
                        id: Some(self.id.to_string()),
                        name: self.meta.name(),
                        version: self.meta.version(),
                        language: self.meta.language(),
                        tags: Some(self.meta.tags()),
                        about
                    }
                };

                let content = toml::ser::to_vec(&package_schema)
                    .map_err(|error| XError::from(("PremierPackageError", error.to_string())))?;

                // Save new package.toml
                File::create(path.join("package.toml"))
                    .await
                    .map_err(|error| XError::from(("PremierPackageError", error.to_string())))?
                    .write(&content)
                    .await
                    .map_err(|error| XError::from(("PremierPackageError", error.to_string())))?;
            }

            // Copy lections files
            for lection in self.lections.iter() {
                match lection.as_any_ref().downcast_ref::<PremierLection>() {
                    Some(lection) => {
                        let lection_path = path.join(PathBuf::from(lection.id()));
                        async_std::fs::create_dir(lection_path.clone())
                            .await
                            .map_err(|error| XError::from(("PremierPackageError", error.to_string())))?;

                        let mut lection_files = Vec::with_capacity(lection.files.collection.len());
                        for lection_file in lection.files.collection.iter() {
                            let lection_filename = lection_file.path.file_name().expect("Unable to get file name from file");
                            let expected_path = lection_path.join(lection_filename);
                            async_std::fs::copy(lection_file.location().await?, expected_path.clone())
                                .await
                                .map_err(|error| XError::from(("PremierPackageError", error.to_string())))?;

                            lection_files.push(crate::schemes::lection::PremierLectionPage {
                                relative: lection_filename.to_string_lossy().to_string()
                            });
                        }

                        let lection_schema = crate::schemes::lection::PremierLection {
                            lection: crate::schemes::lection::PremierLectionMeta {
                                id: Some(lection.id()),
                                name: lection.meta().name(),
                                order: lection.meta().order(),
                                pages: Some(lection_files)
                            }
                        };

                        let content = toml::ser::to_vec(&lection_schema)
                            .map_err(|error| XError::from(("PremierPackageError", error.to_string())))?;

                        // Save new lection.toml
                        File::create(lection_path.join("lection.toml"))
                            .await
                            .map_err(|error| XError::from(("PremierPackageError", error.to_string())))?
                            .write(&content)
                            .await
                            .map_err(|error| XError::from(("PremierPackageError", error.to_string())))?;
                    }
                    None => return XError::from(("Premier package error", "Lection type is not PremierLection. This never suppose to happen")).into()
                }
            }
        }

        // Test saved package
        let package = PremierPackage::load(path.clone())
            .await?;

        // Update paths
        match (self.files.collection.get_mut(&"about".to_string()), package.files.collection.get(&"about".to_string())) {
            (Some(lhs), Some(rhs)) => {
                lhs.path = rhs.path.clone();
                lhs.root = rhs.root.clone();

                // TODO: Possible dangerous assignment
                lhs.uuid = rhs.uuid.clone();
            },
            (None, None) => { /* Fine - no about file */ },
            _ => return XError::from(("PremierPackageError", "Failure on file copy")).into()
        }

        for (lhs, rhs) in self.lections.iter_mut().zip(package.lections.iter()) {
            match (lhs.as_any_mut().downcast_mut::<PremierLection>(), rhs.as_any_ref().downcast_ref::<PremierLection>()) {
                (Some(lhs), Some(rhs)) => {
                    if lhs.files.collection.len() != rhs.files.collection.len() {
                        return XError::from(("PremierPackageError", "Failure on lection copy. Lections have different file set")).into();
                    }
                    lhs.files = rhs.files.clone();
                },
                _ => return XError::from(("PremierPackageError", "Failure on lection copy. Lections have different types")).into()
            }
        }

        Ok(())
    }
}

#[async_trait]
impl DedutyPackage for PremierPackage {
    fn as_any_ref(&self) -> &(dyn std::any::Any + Send + Sync) {
        self
    }

    fn as_any_mut(&mut self) -> &mut (dyn std::any::Any + Send + Sync) {
        self
    }

    fn id(&self) -> String {
        self.id.to_string()
    }


    fn service(&self) -> String {
        "premier".to_string()
    }

    async fn size(&self) -> XResult<Option<usize>> {
        // Rough calculations without package and lection toml files
        let mut size = 0usize;

        // Calculate package files size
        for file in self.files.collection.values() {
            let location = file.location().await?;
            size += async_std::fs::File::open(location.clone())
                .await
                .map_err(|error| XError::from(("PremierPackageError", format!("Unable to open file `{}` due to unexpected error {error}", location.to_string_lossy()))))?
                .metadata()
                .await
                .map_err(|error| XError::from(("PremierPackageError", format!("Unable to get metadata from file `{}` due to unexpected error {error}", location.to_string_lossy()))))?
                .len() as usize;
        }

        // Calculate lection files size
        for lection in self.lections.iter() {
            for file in lection.as_any_ref().downcast_ref::<PremierLection>().unwrap().files.collection.iter() {
                let location = file.location().await?;
                size += async_std::fs::File::open(location.clone())
                    .await
                    .map_err(|error| XError::from(("PremierPackageError", format!("Unable to open file `{}` due to unexpected error {error}", location.to_string_lossy()))))?
                    .metadata()
                    .await
                    .map_err(|error| XError::from(("PremierPackageError", format!("Unable to get metadata from file `{}` due to unexpected error {error}", location.to_string_lossy()))))?
                    .len() as usize;
            }
        }

        Ok(Some(size))
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
