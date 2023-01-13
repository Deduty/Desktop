use std::collections::HashMap;

use uuid::Uuid;

pub enum FileAffinity {
    Package { package: Uuid},
    Lection { package: Uuid, lection: Uuid }
}


pub struct FilePackageIndex {
    index: HashMap<Uuid, FileAffinity>
}

impl FilePackageIndex {
    pub fn new() -> Self {
        Self { index: HashMap::new() }
    }

    pub fn index(&mut self) -> &mut HashMap<Uuid, FileAffinity> {
        &mut self.index
    }
}
