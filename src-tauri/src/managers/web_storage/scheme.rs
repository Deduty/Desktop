use std::collections::HashMap;

use async_std::io::WriteExt;
use async_std::{fs::File, io::ReadExt};
use async_std::path::PathBuf;
use serde::{ Deserialize, Serialize };

use xresult::{ XError, XResult, XReason };


#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WebStorageScheme {
    package: HashMap<String, String>,
    lections: HashMap<String, HashMap<String, String>>
}

impl WebStorageScheme {
    pub fn package_ref(&self) -> &HashMap<String, String> {
        &self.package
    }

    pub fn package_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.package
    }

    pub fn lection_ref(&self) -> &HashMap<String, HashMap<String, String>> {
        &self.lections
    }

    pub fn lection_mut(&mut self) -> &mut HashMap<String, HashMap<String, String>> {
        &mut self.lections
    }

    pub async fn from(path: PathBuf) -> XResult<Self> {
        if path.is_dir().await {
            return XError::from(("Storage error", format!("Path `{}` must be file path or not exist", path.to_string_lossy()))).into();
        }

        if !path.exists().await {
            return Ok(Default::default());
        }

        let mut file = File::open(path)
            .await
            .map_err(|error| XError::from(("Storage error", error.to_string())))?;

        let mut buffer = vec![];
        file
            .read_to_end(&mut buffer)
            .await
            .map_err(|error| XError::from(("Storage error", error.to_string())))?;

        Ok(
            serde_json::from_slice(&buffer)
                .map_err(|error| XError::from(("Storage error", error.to_string())))?
        )
    }

    pub async fn save(&self, path: PathBuf) -> XReason {
        let buffer = serde_json::to_vec(&self)
            .map_err(|error| XError::from(("Storage error", error.to_string())))?;

        let mut file = File::create(path.clone())
            .await
            .map_err(|error| XError::from(("Storage error", error.to_string())))?;

        file.write(&buffer)
            .await
            .map_err(|error| XError::from(("Storage error", error.to_string())))?;

        Ok(())
    }
}
