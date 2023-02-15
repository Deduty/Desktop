use async_std::path::PathBuf;
use serde::Serialize;
use xresult::{ XResult, XReason };

mod meta;
mod read;
mod unique;


fn os_to_string(os: &std::ffi::OsStr) -> Option<String> {
    os.to_str().map(ToString::to_string)
}


#[derive(Debug, Serialize)]
struct AutoFileMeta {
    name: Option<String>,
    ext: Option<String>
}


#[derive(Debug)]
pub struct AutoFile {
    id: String,
    size: usize,
    path: PathBuf,

    origin: Option<String>,
    ext: Option<String>,

    meta: String
}

impl AutoFile {
    pub async fn load(path: PathBuf) -> XResult<Self> {
        if !path.is_file().await {
            return crate::error::error_err(format!("Path '{path:#?}' is not a file"))
        }

        let id = uuid::Uuid::new_v4().to_string();
        let size = match path.metadata().await {
            Ok(metadata) => metadata.len() as usize,
            Err(error) => return crate::error::error_err(format!("Unable to read metadata of file '{path:#?}': {error}"))
        };

        let origin = path.file_name().and_then(os_to_string);
        let ext = path.extension().and_then(os_to_string);

        let meta = match serde_json::to_string(&AutoFileMeta { name: origin.clone(), ext: ext.clone() }) {
            Ok(meta) => meta,
            Err(error) => return crate::error::error_err(format!("Unable to prepare meta for file '{path:#?}': {error}"))
        };

        Ok(AutoFile { id, size, path, origin, ext, meta })
    }

    /// Doesn't creates any dirs
    pub async fn save(&self, root: &PathBuf) -> XReason {
        if self.path.starts_with(root) {
            return Ok(())
        }

        let filepath = root.join(self.filename());
        async_std::fs::copy(&self.path, &filepath)
            .await
            .map_err(|error|
                crate::error::error(format!("Unable to save file from {:#?} to {filepath:#?}: {error}", &self.path)))?;

        Ok(())
    }

    pub fn filename(&self) -> String {
        match (&self.origin, &self.ext) {
            (Some(origin), Some(ext)) => format!("{origin}.{ext}"),
            (None, Some(ext)) => format!("{}.{ext}", self.id),
            _ => self.id.clone(),
        }
    }
}
