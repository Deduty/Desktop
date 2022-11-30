use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

use async_std::fs::File;
use async_std::path::PathBuf;
use async_trait::async_trait;

use package::file::traits::DedutyFile;

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

#[derive(Debug)]
pub struct PremierFileError {
    message: String
}

impl PremierFileError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Error for PremierFileError {}
impl Display for PremierFileError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_fmt(format_args!("Internal error: {}", self.message))
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

    fn location(&self) -> PathBuf {
        match self.path.is_absolute() {
            true => self.path.clone(),
            false => {
                // TODO: ASYNC, may PANIC
                // self.root.join(self.path.clone()).canonicalize().await.unwrap()

                // SYNC WAY AROUND
                let std_root = std::path::PathBuf::from_str(self.root.as_os_str().to_string_lossy().as_ref()).unwrap();
                let std_path = std::path::PathBuf::from_str(self.path.as_os_str().to_string_lossy().as_ref()).unwrap();
                let std_absolute = std_root
                    .join(std_path.as_path())
                    .canonicalize()
                    .unwrap();

                PathBuf::from_str(std_absolute.as_os_str().to_string_lossy().as_ref()).unwrap()
             }
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

    async fn load(&self) -> Result<File, Box<dyn Error + Send>> {
        let target = self.location();
        if !target.exists().await {
            return Err(
                Box::new(
                    PremierFileError::new(
                        format!(
                            "Location not exist: '{}'",
                            target.as_os_str()
                                .to_str()
                                .unwrap_or("<error>")))));
        }
        if !target.is_file().await {
            return Err(
                Box::new(
                    PremierFileError::new(
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
                    PremierFileError::new(
                        format!(
                            "Error occurred while opening file at location '{}':\n {}",
                            target.as_os_str()
                                .to_str()
                                .unwrap_or("<error>"),
                            error.to_string())))
                as Box<dyn Error + Send>
            )
    }
}
