use std::collections::HashMap;

use async_std::{
    fs::File,
    io::{ ReadExt, WriteExt },
    path::PathBuf,
    stream::StreamExt,
    sync::Arc
};
use deduty_package::{ MetaFile, UniqueFile };
use serde::{ Deserialize, Serialize };
use xresult::{ XReason, XResult };

use crate::schemes::{
    Lection,
    lection::LectionFiles
};

use crate::file::AutoFile;

mod meta;
mod peek;
mod read;
mod unique;


fn os_to_string(os: &std::ffi::OsStr) -> Option<String> {
    os.to_str().map(ToString::to_string)
}


#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AutoLectionMeta {
    pub name: String,
    pub hidden: bool
}


#[derive(Debug)]
pub struct AutoLection {
    id: String,    
    size: usize,
    path: PathBuf,

    name: String,
    origin: Option<String>,

    meta: String,

    files_index: HashMap<String, Arc<AutoFile>>,
    files_order: Vec<Arc<AutoFile>>,
}

impl AutoLection {
    pub async fn load(path: PathBuf, is_about: bool) -> XResult<Self> {
        if !path.is_dir().await {
            return crate::error::error_err(format!("Path `{path:#?}` is not a directory"));
        }

        let mut id = match is_about {
            true => "about".to_string(),
            false => uuid::Uuid::new_v4().to_string()
        };

        let mut name = None;
        let origin = path.file_name().and_then(os_to_string);


        let lection: Option<Lection> = {
            let lection_toml = path.join("lection.toml");

            let mut buffer = Vec::new();
            File::open(&lection_toml)
                .await
                .map_err(|error|
                    crate::error::error(format!("Unable to open file `{lection_toml:#?}`: {error}")))?
                .read_to_end(&mut buffer)
                .await
                .map_err(|error|
                    crate::error::error(format!("Unable to read file `{lection_toml:#?}`: {error}")))?;
    
            Some(
                toml::from_slice(&buffer)
                    .map_err(|error|
                        crate::error::error(format!("Unable to get lection meta from `{lection_toml:#?}`: {error}")))?
            )
        };

        let mut files = Vec::new();
        {
            let mut entries = async_std::fs::read_dir(&path)
                .await
                .map_err(|error| crate::error::error(format!("Unable to scan directory {path:#?}: {error}")))?;

            while let Some(entry) = entries.next().await {
                let Ok(path) = entry.map(|entry| entry.path()) else {
                    continue;
                };

                let Some(name) = path.file_name().and_then(os_to_string) else {
                    continue;
                };

                if !path.is_file().await {
                    continue;
                }

                files.push((name, Arc::new(AutoFile::load(path).await?)));
            }
        }

        {
            let mut first_names = HashMap::new();
            let mut last_names = HashMap::new();

            if let Some(lection) = lection.and_then(|lection| lection.lection) {
                id = lection.id.unwrap_or(id);
                name = lection.name.or_else(|| origin.clone());

                if let Some(first) = lection.first {
                    for (order, name) in first.into_iter().enumerate() {
                        first_names.insert(name, order);
                    }
                }

                if let Some(last) = lection.last {
                    for (order, name) in last.into_iter().enumerate() {
                        last_names.insert(name, order);
                    }
                }
            }

            files.sort_by(|(lhs, _), (rhs, _)| {
                match (first_names.get(lhs), last_names.get(lhs), first_names.get(rhs), last_names.get(rhs)) {
                    // <l, >l, <r, >r

                    // <l, <r
                    // Both first -> compare positions in first array
                    (Some(lhs_first), None, Some(rhs_first), None) => lhs_first.cmp(rhs_first),

                    // >l, >r
                    // Both last -> compare positions in last array
                    (None, Some(lhs_last), None, Some(rhs_last)) => lhs_last.cmp(rhs_last),

                    // <l, >r
                    // Lhs in first, rhs in last
                    (Some(_), None, None, Some(_)) => std::cmp::Ordering::Less,

                    // >l, <r
                    // Lhs in last, rhs in first
                    (None, Some(_), Some(_), None) => std::cmp::Ordering::Greater,

                    // Any strange case and no order case -> cmp by name
                    _ => lhs.cmp(rhs)
                }
            });
        }

        let mut size = 0usize;
        for (_, file) in files.iter() {
            size += file.size().expect("Auto service must know file size");
        }

        let name = name.unwrap_or_else(|| id.clone());

        
        let meta = serde_json::to_string(&AutoLectionMeta { name: name.to_string(), hidden: is_about })
            .map_err(|error| crate::error::error(format!("Unable to serialize meta for `{path:#?}`: {error}")))?;

        Ok(Self {
            id,
            size,
            path,

            name,
            origin,

            meta,

            files_index: files.iter().map(|(_, file)| (file.id().to_string(), file.clone())).collect(),
            files_order: files.into_iter().map(|(_, file)| file).collect()
        })
    }

    /// Ensures lection dir existence
    pub async fn save(&self, root: &PathBuf) -> XReason {
        if self.path.starts_with(root) {
            return Ok(())
        }

        let dirname = match &self.origin {
            Some(origin) => origin,
            None => &self.id
        };
        let lection_root = root.join(dirname);

        // Safe since `create_dir` return error on path existence
        let _  = async_std::fs::create_dir(&lection_root).await;

        // Create lection.toml file
        let lection_toml_content = Lection {
            lection: Some(LectionFiles {
                id: Some(self.id.to_string()),
                name: Some(self.name.to_string()),
                first: Some(self.files_order.iter().map(|file| file.filename()).collect()),
                last: None
            })
        };

        let serialized = toml::to_vec(&lection_toml_content)
            .map_err(|error|
                crate::error::error(format!("Unable to serialize lection meta for `{}`: {error}", self.id)))?;

        File::create(lection_root.join("lection.toml"))
            .await
            .map_err(|error|
                crate::error::error(format!("Unable to save metadata for `{}`: {error}", self.id)))?
            .write_all(serialized.as_slice())
            .await
            .map_err(|error|
                crate::error::error(format!("Unable to write metadata for `{}`: {error}", self.id)))?;

        for file in self.files_order.iter() {
            file.save(&lection_root).await?;
        }

        Ok(())
    }
}
