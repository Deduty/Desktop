use std::sync::Arc;

use async_std::sync::RwLock;
use async_trait::async_trait;

use xresult::{ XReason, XResult };

use crate::agent::PackageAgent;


type SafePackageAgent = Arc<RwLock<Box<dyn PackageAgent>>>;


#[async_trait]
pub trait IndexService: Send + Sync {
    async fn load_all(&mut self) -> XResult<Vec<XReason>>;
    async fn save_all(&mut self) -> XResult<Vec<XReason>>;

    async fn add(&mut self, serialized: String) -> XResult<SafePackageAgent>;
    async fn get(&self, id: &String) -> XResult<Option<SafePackageAgent>>;
    async fn has(&self, id: &String) -> XResult<bool>;
    async fn list(&self) -> XResult<Vec<String>>;
    async fn sub(&mut self, id: &String) -> XReason;
    // TODO: Update accepts HashMap<String, FrontEndSerialization>
    //       Where FES is String that served by update method.
    //       Requirements gives HashMap<String, FrontEndRequirement>
    //       Where FER is ZeroSizedType which can only be serialized.
    async fn update(&mut self, serialized: String, id: &String) -> XReason;
}
