use serde::Deserialize;

pub enum PackageManifestVariants {
    Premier
}

impl PackageManifestVariants {
    pub fn to_string(&self) -> String {
        match &self {
            PackageManifestVariants::Premier => "premier".to_string()
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
