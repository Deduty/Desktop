use std::collections::HashMap;

use async_std::io::WriteExt;
use async_std::{fs::File, io::ReadExt};
use async_std::path::PathBuf;
use serde::{ Deserialize, Serialize };

use xresult::{ XError, XResult, XReason };

#[derive(Debug, Clone, Deserialize, Serialize)]
struct SerdeDedutyPackageStorage {
    package: HashMap<String, String>,
    lection: HashMap<String, HashMap<String, String>>
}

pub struct DedutyPackageStorage {
    path: PathBuf,
    package: HashMap<String, String>,
    lection: HashMap<String, HashMap<String, String>>
}

impl DedutyPackageStorage {
    pub fn package_ref(&self) -> &HashMap<String, String> {
        &self.package
    }

    pub fn package_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.package
    }

    pub fn lections_ref(&self) -> &HashMap<String, HashMap<String, String>> {
        &self.lection
    }

    pub fn lections_mut(&mut self) -> &mut HashMap<String, HashMap<String, String>> {
        &mut self.lection
    }

    pub async fn from(path: PathBuf) -> XResult<Self> {
        if path.is_dir().await {
            return Err(XError::from((
                "Deduty package storage error",
                format!("Path {} must not be dir path (it must be file or non-exist)", path.to_string_lossy())
            )).into());
        }

        match path.exists().await {
            false => Ok(Self { path, package: HashMap::new(), lection: HashMap::new() }),
            true => {
                let mut file = File::open(path.clone())
                    .await
                    .map_err(|error| XError::from(("Deduty package storage error", error.to_string())))?;

                let mut buffer = vec![];
                file
                    .read_to_end(&mut buffer)
                    .await
                    .map_err(|error| XError::from(("Deduty package storage error", error.to_string())))?;

                let schema: SerdeDedutyPackageStorage = serde_json::from_slice(&buffer)
                    .map_err(|error| XError::from(("Deduty package storage error", error.to_string())))?;

                Ok(Self { path, package: schema.package, lection: schema.lection })
            }
        }
    }

    pub async fn save(&self) -> XReason {
        let schema = SerdeDedutyPackageStorage { package: self.package.clone(), lection: self.lection.clone() };
        let buffer = serde_json::to_vec(&schema)
            .map_err(|error| XError::from(("Deduty package storage error", error.to_string())))?;

        let mut file = File::create(self.path.clone())
            .await
            .map_err(|error| XError::from(("Deduty package storage error", error.to_string())))?;

        file.write(&buffer)
            .await
            .map_err(|error| XError::from(("Deduty package storage error", error.to_string())))?;

        Ok(())
    }
}
