use std::sync::Arc;

use async_std::path::PathBuf;
use async_std::sync::RwLock;
use async_trait::async_trait;

use xresult::{ XReason, XResult };

use crate::agent::PackageAgent;


type SafePackageAgent = Arc<RwLock<Box<dyn PackageAgent>>>;


#[async_trait]
pub trait IndexService: Send + Sync {
    async fn load_all(&mut self, root: &PathBuf) -> Vec<XReason>;
    async fn save_all(&mut self, root: &PathBuf) -> Vec<XReason>;

    async fn add(&mut self, serialized: String, id: Option<String>) -> XResult<SafePackageAgent>;
    async fn get(&self, id: &String) -> XResult<Option<SafePackageAgent>>;
    async fn list(&self) -> XResult<Vec<String>>;
    async fn sub(&mut self, id: &String) -> XReason;
    async fn update(&mut self, serialized: String, id: &String) -> XReason;
}
