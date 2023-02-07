use async_std::path::PathBuf;
use directories::ProjectDirs;

use xresult::{ XError, XReason, XResult };


#[derive(Clone, Debug)]
pub struct Settings {
    project: ProjectDirs
}

impl Settings {
    pub async fn create() -> XResult<Self> {
        let project = ProjectDirs::from("edu", "Deduty", "Deduty Desktop")
            .ok_or(XError::from(("Deduty settings error", "Unable to get project directories")))?;

        let settings = Self { project };
        settings.ensure_existence().await?;

        Ok(settings)
    }

    pub async fn ensure_existence(&self) -> XReason {
        async_std::fs::create_dir_all(self.project.data_dir())
            .await
            .map_err(|error| XError::from(("Deduty settings error", error.to_string())))?;

        async_std::fs::create_dir_all(self.project.preference_dir())
            .await
            .map_err(|error| XError::from(("Deduty settings error", error.to_string())))?;
        
        Ok(())
    }

    pub fn services(&self) -> PathBuf {
        PathBuf::from(self.project.data_dir()).join("services")
    }

    pub fn resources(&self) -> PathBuf {
        PathBuf::from(self.project.data_dir())
    }

    pub fn settings(&self) -> PathBuf {
        PathBuf::from(self.project.preference_dir())
    }
}
