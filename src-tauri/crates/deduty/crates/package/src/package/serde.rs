use serde::Serialize;

use crate::file::serde::DedutyFileCollection;

#[derive(Serialize)]
pub struct DedutyPackageMeta {
    pub name: String,
    pub version: String,
    pub language: String,
    pub tags: Vec<String>
}

#[derive(Serialize)]
pub struct DedutyPackage {
    pub id: String,
    pub meta: DedutyPackageMeta,
    pub files: DedutyFileCollection
}
