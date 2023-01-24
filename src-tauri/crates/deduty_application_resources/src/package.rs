use std::collections::HashMap;

pub enum FileAffinity {
    Package { package: String },
    Lection { package: String, lection: String }
}


pub struct FilePackageIndex {
    index: HashMap<String, FileAffinity>
}

impl FilePackageIndex {
    pub fn new() -> Self {
        Self { index: HashMap::new() }
    }

    pub fn index(&mut self) -> &mut HashMap<String, FileAffinity> {
        &mut self.index
    }
}
