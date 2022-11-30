pub mod serde;
pub mod traits;

impl From<&dyn traits::DedutyFile> for serde::DedutyFile {
    fn from(object: &dyn traits::DedutyFile) -> Self {
        Self {
            alias: object.alias().cloned(),
            location: object.location().clone().to_string_lossy().to_string(),
            extension: object.extension().clone(),
        }
    }
}

impl From<&dyn traits::DedutyFileCollection> for serde::DedutyFileCollection {
    fn from(object: &dyn traits::DedutyFileCollection) -> Self {
        Self {
            files: object.files().iter().map(|&file| file.into()).collect()
        }
    }
}
