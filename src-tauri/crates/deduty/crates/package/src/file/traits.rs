use async_std::fs::File;
use async_std::path::{Path, PathBuf};
use async_trait::async_trait;

use crate::error::XResult;


#[async_trait]
pub trait DedutyFile: Sync + Send {
    fn alias(&self) -> Option<&String>;
    fn extension(&self) -> String;
    async fn location(&self) -> XResult<PathBuf>;
    async fn load(&self) -> XResult<File>;
}

#[async_trait]
pub trait DedutyFileCollection: Sync + Send {
    async fn alias(&self, alias: &String) -> XResult<Option<&dyn DedutyFile>>;
    async fn file(&self, path: &Path) -> XResult<Option<&dyn DedutyFile>>;
    async fn files(&self) -> XResult<Vec<&dyn DedutyFile>>;
}
