use std::collections::HashMap;
use std::sync::Arc;

use async_std::sync::RwLock;
use uuid::Uuid;

use deduty_package::DedutyFileReader;


#[derive(Default)]
pub struct ReaderManager {
    readers: RwLock<HashMap<String, Arc<RwLock<Box<dyn DedutyFileReader>>>>>
}

impl ReaderManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn add(&self, reader: Box<dyn DedutyFileReader>) -> String {
        let mut readers = self.readers.write().await;
        let mut token = Uuid::new_v4().to_string();

        // Note: This is very rare case, but I decided to cover it.
        while readers.contains_key(&token) {
            token = Uuid::new_v4().to_string();
        }

        readers.insert(token.clone(), Arc::new(RwLock::new(reader)));
        token
    }

    pub async fn get(&self, token: &str) -> Option<Arc<RwLock<Box<dyn DedutyFileReader>>>> {
        self.readers.write().await.get_mut(token).cloned()
    }
}
