use serde::{ Deserialize, Serialize };


#[derive(Debug, Deserialize, Serialize)]
pub struct LectionsExactItem {
    pub relative: String
}


#[derive(Debug, Deserialize, Serialize)]
pub struct LectionsMeta {
    pub regex: Option<String>,
    pub exact: Option<Vec<LectionsExactItem>>
}


#[derive(Debug, Deserialize, Serialize)]
pub struct PackageMeta {
    pub id: Option<String>,
    pub name: String,
    pub version: String,
    pub language: String,
    pub about: Option<String>,
    pub tags: Option<Vec<String>>
}


#[derive(Debug, Deserialize, Serialize)]
pub struct PremierPackage {
    pub package: PackageMeta,
    pub lections: LectionsMeta
}
