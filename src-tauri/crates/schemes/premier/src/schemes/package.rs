use serde::Deserialize;


#[derive(Deserialize)]
pub struct LectionsExactItem {
    pub relative: String
}


#[derive(Deserialize)]
pub struct LectionsMeta {
    pub regex: Option<String>,
    pub exact: Option<Vec<LectionsExactItem>>
}


#[derive(Deserialize)]
pub struct PackageMeta {
    pub name: String,
    pub version: String,
    pub language: String,
    pub about: Option<String>,
    pub tags: Option<Vec<String>>
}


#[derive(Deserialize)]
pub struct PremierPackage {
    pub package: PackageMeta,
    pub lections: LectionsMeta
}
