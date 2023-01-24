use async_std::fs::File;
use async_std::path::PathBuf;
use async_trait::async_trait;
use uuid::Uuid;

use deduty_package_traits::{ DedutyFile, DedutyFileReader };
use xresult::{ XError, XResult };

use super::reader::PremierFileReader;


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
    uuid: Uuid
}

impl PremierFile {
    pub fn new(alias: PremierFileAlias, root: PathBuf, path: PathBuf, uuid: Uuid) -> Self {
        Self { alias: alias.into(), root, path, uuid }
    }

    pub fn from(alias: PremierFileAlias, root: &String, path: &String, uuid: Uuid) -> Self {
        Self::new(alias, PathBuf::from(root), PathBuf::from(path), uuid)
    }

    pub async fn location(&self) -> XResult<PathBuf> {
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
}

impl From<PremierFile> for Box<dyn DedutyFile> {
    fn from(file: PremierFile) -> Self {
        Box::new(file) as Box<dyn DedutyFile>
    }
}

#[async_trait]
impl DedutyFile for PremierFile {
    fn alias(&self) -> Option<String> {
        self.alias.clone()
    }

    fn extension(&self) -> String {
        self.path
            .extension()
            .map(|os_str| os_str.to_string_lossy().to_string())
            .unwrap_or("".into())
    }

    fn id(&self) -> String {
        self.uuid.to_string()
    }

    async fn load(&self) -> XResult<Box<dyn DedutyFileReader>> {
        let target = self.location().await?;
        if !target.exists().await {
            let location = target.as_os_str().to_str().unwrap_or("<error>");
            return Err(Box::new(XError::from(("DedutyFileError", format!("Location does not exist: `{}`", location)))))
        }
        if !target.is_file().await {
            let location = target.as_os_str().to_str().unwrap_or("<error>");
            return Err(Box::new(XError::from(("DedutyFileError", format!("Location is not a folder: `{}`", location)))));
        }

        let file = File::open(target.clone())
            .await
            .map_err(|error| Box::new(XError::from(("DedutyFileError", error.to_string()))))?;
        
        Ok(Box::new(PremierFileReader::new(file)))
    }
}
