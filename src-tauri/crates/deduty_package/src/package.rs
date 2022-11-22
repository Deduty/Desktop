use std::convert::From;

use serde::Serialize;

#[derive(Serialize)]
pub struct SerializedDedutyPackage {
    pub name: String,
    pub version: String,
    pub language: String
}

impl From<Box<dyn DedutyPackage>> for SerializedDedutyPackage {
    fn from(package: Box<dyn DedutyPackage>) -> Self {
        SerializedDedutyPackage {
            name: package.name().clone(),
            version: package.version().clone(),
            language: package.language().clone()
        }
    }
}

pub trait DedutyPackage {
    fn name(&self) -> &String;
    fn version(&self) -> &String;
    fn language(&self) -> &String;
}
