use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

use async_std::sync::RwLock;
use uuid::Uuid;

use deduty_package::DedutyFileReader;


type ReaderRef = Box<dyn Deref<Target = dyn DedutyFileReader> + Send + Sync>;

fn into_reader_ref(reader: Arc<dyn DedutyFileReader>) -> ReaderRef {
    Box::new(reader) as ReaderRef
}


#[derive(Default)]
pub struct ReaderManager {
    readers: RwLock<HashMap<String, Arc<dyn DedutyFileReader>>>
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

        readers.insert(token.clone(), Arc::from(reader));
        token
    }

    pub async fn get(&self, token: &str) -> Option<ReaderRef> {
        self.readers.read().await.get(token).cloned().map(into_reader_ref)
    }

    pub async fn sub(&self, token: &str) -> Option<ReaderRef> {
        self.readers.write().await.remove(token).map(into_reader_ref)
    }
}
