use async_std::fs::File;
use async_std::path::PathBuf;
use async_trait::async_trait;

use package::file::traits::DedutyFile;

use crate::error::{ PremierError, XResult };


pub enum PremierFileAlias {
    Alias(String),
    NoAlias
}

impl Into<Option<String>> for PremierFileAlias {
    fn into(self) -> Option<String> {
        match self {
            PremierFileAlias::Alias(string) => Some(string),
            PremierFileAlias::NoAlias => None
        }
    }
}


pub struct PremierFile {
    alias: Option<String>,
    root: PathBuf,
    path: PathBuf,
}

impl PremierFile {
    pub fn new(alias: PremierFileAlias, root: PathBuf, path: PathBuf) -> Self {

        Self { alias: alias.into(), root, path }
    }

    pub fn from(alias: PremierFileAlias, root: &String, path: &String) -> Self {
        Self::new(alias, PathBuf::from(root), PathBuf::from(path))
    }
}

impl From<PremierFile> for Box<dyn DedutyFile> {
    fn from(file: PremierFile) -> Self {
        Box::new(file) as Box<dyn DedutyFile>
    }
}

#[async_trait]
impl DedutyFile for PremierFile {
    fn alias(&self) -> Option<&String> {
        self.alias.as_ref()
    }

    async fn location(&self) -> XResult<PathBuf> {
        match self.path.is_absolute() {
            true => Ok(self.path.clone()),
            false => Ok(self.root.join(self.path.clone()))
                //
                // CANON WAY (COULD NOT FOR NOW)
                // .canonicalize()
                // .await
                // .map_err(|err|
                //     Box::new(
                //         PremierError::new(
                //             format!("Internal error: Unable to resolve location due to: {}",
                //             err.to_string())))
                //     .into())
        }
    }

    fn extension(&self) -> String {
        self.path
            .extension()
            .map(|os_str|
                os_str
                    .to_string_lossy()
                    .to_string())
            .unwrap_or("".into())
    }

    async fn load(&self) -> XResult<File> {
        let target = self.location().await?;
        if !target.exists().await {
            return Err(
                Box::new(
                    PremierError::new(
                        format!(
                            "Location not exist: '{}'",
                            target.as_os_str()
                                .to_str()
                                .unwrap_or("<error>")))));
        }
        if !target.is_file().await {
            return Err(
                Box::new(
                    PremierError::new(
                        format!(
                            "Location is not a folder: '{}'",
                            target.as_os_str()
                                .to_str()
                                .unwrap_or("<error>")))));
        }

        File::open(target.clone())
            .await
            .map_err(|error|
                Box::new(
                    PremierError::new(
                        format!(
                            "Error occurred while opening file at location '{}':\n {}",
                            target.as_os_str()
                                .to_str()
                                .unwrap_or("<error>"),
                            error.to_string())))
                .into()
            )
    }
}
