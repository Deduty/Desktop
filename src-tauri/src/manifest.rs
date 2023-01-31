use std::fmt::Display;

use serde::Deserialize;

pub enum PackageManifestVariants {
    Premier
}

impl Display for PackageManifestVariants {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PackageManifestVariants::Premier => f.write_str("premier")
        }
    }
}


#[derive(Deserialize)]
struct Manifest {
    name: String
}

#[derive(Deserialize)]
pub struct PackageManifest {
    manifest: Manifest
}

impl PackageManifest {
    pub fn as_string(&self) -> &String {
        &self.manifest.name
    }

    pub fn to_enum(&self) -> Option<PackageManifestVariants> {
        match self.manifest.name.to_lowercase().as_str() {
            "premier" => Some(PackageManifestVariants::Premier),
            _ => None
        }
    }
}
