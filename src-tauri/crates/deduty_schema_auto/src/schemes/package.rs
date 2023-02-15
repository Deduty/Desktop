use serde::{ Deserialize, Serialize };


#[derive(Debug, Deserialize, Serialize)]
pub struct PackageLections {
    pub first: Option<Vec<String>>,
    pub last: Option<Vec<String>>
}


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PackageMeta {
    pub id: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub language: Option<String>,
    pub about: Option<String>,
    pub tags: Option<Vec<String>>
}


#[derive(Debug, Deserialize, Serialize)]
pub struct PackageManifest {
    pub name: String
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Package {
    pub manifest: PackageManifest,
    pub package: Option<PackageMeta>,
    pub lection: Option<PackageLections>
}
