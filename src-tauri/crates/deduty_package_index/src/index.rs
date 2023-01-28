use std::collections::HashMap;

use crate::service::IndexService;


pub struct DedutyPackageIndex {
    services: HashMap<String, Box<dyn IndexService>>
}

impl DedutyPackageIndex {
    pub fn new() -> Self {
        Self { services: HashMap::new() }
    }

    pub fn from(services: HashMap<String, Box<dyn IndexService>>) -> Self {
        Self { services }
    }

    pub fn services_ref(&self) -> &HashMap<String, Box<dyn IndexService>> {
        &self.services
    }

    pub fn services_mut(&mut self) -> &mut HashMap<String, Box<dyn IndexService>> {
        &mut self.services
    }
}
