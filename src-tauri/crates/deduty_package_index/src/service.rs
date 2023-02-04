use std::collections::HashMap;
use std::sync::Arc;

use async_std::sync::RwLock;
use async_trait::async_trait;

use xresult::{ XReason, XResult };

use crate::agent::PackageAgent;
use crate::reqs::{
    FrontEndRequirement as FER,
    FrontEndSerialization as FES
};


type SafePackageAgent = Arc<RwLock<Box<dyn PackageAgent>>>;


#[async_trait]
pub trait IndexService: Send + Sync {
    async fn load_all(&mut self) -> XResult<Vec<XReason>>;
    async fn save_all(&mut self) -> XResult<Vec<XReason>>;

    async fn get(&self, id: &String) -> XResult<Option<SafePackageAgent>>;
    async fn has(&self, id: &String) -> XResult<bool>;
    async fn list(&self) -> XResult<Vec<String>>;
    async fn sub(&mut self, id: &String) -> XReason;

    async fn update(&mut self, serialized: HashMap<String, FES>, id: &String) -> XReason;
    fn update_reqs(&self) -> &HashMap<String, FER>;

    async fn add(&mut self, serialized: HashMap<String, FES>) -> XResult<SafePackageAgent>;
    fn add_reqs(&self) -> &HashMap<String, FER>;
}
