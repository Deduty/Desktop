use async_std::path::PathBuf;
use directories::ProjectDirs;

use xresult::{ XError, XResult };


#[derive(Clone, Debug)]
pub struct Settings {
    project: ProjectDirs
}

impl Settings {
    pub fn new() -> XResult<Self> {
        Ok(Self {
            project: ProjectDirs::from("edu", "Deduty", "Deduty Desktop")
                .ok_or(XError::from(("Deduty settings error", "Unable to get project directories")))?
        })
    }

    pub fn resources(&self) -> PathBuf {
        PathBuf::from(self.project.data_dir())
    }

    pub fn settings(&self) -> PathBuf {
        PathBuf::from(self.project.preference_dir())
    }
}
