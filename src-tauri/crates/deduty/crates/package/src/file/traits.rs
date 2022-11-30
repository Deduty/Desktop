use std::error::Error;

use async_std::fs::File;
use async_std::path::{Path, PathBuf};
use async_trait::async_trait;


#[async_trait]
pub trait DedutyFile: Sync + Send {
    fn alias(&self) -> Option<&String>;
    fn location(&self) -> PathBuf;
    fn extension(&self) -> String;
    async fn load(&self) -> Result<File, Box<dyn Error + Send>>;
}

pub trait DedutyFileCollection: Sync + Send {
    fn alias(&self, alias: &String) -> Option<&dyn DedutyFile>;
    fn file(&self, path: &Path) -> Option<&dyn DedutyFile>;
    fn files(&self) -> Vec<&dyn DedutyFile>;
}
