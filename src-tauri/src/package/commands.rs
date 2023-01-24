use std::collections::{ HashMap, HashSet };
use std::str::FromStr;
use std::sync::Arc;

use async_std::io::ReadExt;
use async_std::fs::File;
use async_std::sync::RwLock;
use async_std::path::Path;
use uuid::Uuid;

use deduty_package_traits::DedutyPackage;
use deduty_package_serde::{
    SerdeDedutyPackage,
    SerdeDedutyLection
};

use deduty_package_index::active::{ ActivePackage, ActiveStorage };

use deduty_application_resources::package::{
    FileAffinity,
    FilePackageIndex
};
use deduty_application_resources::reader::FileReaderIndex;

type StateActiveStorage<'l> = tauri::State<'l, Arc<ActiveStorage>>;
type StateFilePackageIndex<'l> = tauri::State<'l, Arc<RwLock<FilePackageIndex>>>;
type StateFileReaderIndex<'l> = tauri::State<'l, Arc<RwLock<FileReaderIndex>>>;


#[tauri::command]
pub async fn addLocalPackage<'s>(storage: StateActiveStorage<'s>, index: StateFilePackageIndex<'s>, path: &str) -> Result<String, String> {
    // PATH
	let path = Path::new(path);
    if !path.exists().await {
        return Err(format!("Path '{:#?}' is not exist", path));
    }
  	if !path.is_dir().await {
    	return Err(format!("Path '{:#?}' is not a directory", path));
  	}

    // PACKAGE
    let package_toml = path.join("package.toml");
	if !package_toml.exists().await {
    	return Err("'package.toml' is not exist".into());
  	}
	if !package_toml.is_file().await {
		return Err("'package.toml' is not a file".into());
	}

    let package_toml_content = {
        let mut buffer = Vec::new();
        File::open(package_toml.as_path())
            .await
            .map_err(|error| error.to_string())?
            .read_to_end(&mut buffer)
            .await
            .map_err(|error| error.to_string())?;
        buffer
    };

    // PACKAGE MANIFEST
    let manifest: super::manifest::PackageManifest = 
        toml::from_slice(&package_toml_content)
            .map_err(|error| format!("Internal error: {}", error.to_string()))?;

    match manifest.to_enum() {
        // PREMIER PACKAGE
        Some(super::manifest::PackageManifestVariants::Premier) => {
            let package = deduty_scheme_premier::actions::load(path.into())
                .await
                .map_err(|error| format!("Internal error: {}", error.to_string()))?;
            let uuid = package.id().clone();

            // COLLISION CHECK
            // CREATING VIRTUAL INDEX
            let mut virtual_index = HashMap::new();
            // COLLECT PACKAGES IDs FOR VIRTUAL INDEX
            match package.files().files().await {
                Err(error) => return Err(format!("Unable to update program index for `{:?}`: {}", path, error.to_string())),
                Ok(files) => {
                    for file in files {
                        virtual_index.insert(file.id(), FileAffinity::Package { package: uuid.clone() });
                    }
                }
            }
            for lection in package.lections() {
                match lection.files().files().await {
                    Err(error) => return Err(format!("Unable to update program index for `{:?}`: {}", path, error.to_string())),
                    Ok(files) => {
                        for file in files {
                            virtual_index.insert(file.id(), FileAffinity::Lection { package: uuid.clone(), lection: lection.id().clone() });
                        }
                    }
                }
            }

            // COLLISION CHECK BEGIN
            let mut rwlock_index = index.write().await;
            let virtual_index_keys = virtual_index.keys().collect::<HashSet<_>>();
            let package_index_keys = rwlock_index.index().keys().collect::<HashSet<_>>();

            let intersected = package_index_keys.intersection(&virtual_index_keys).collect::<Vec<_>>();
            if intersected.len() > 0 {
                return Err(format!("Unable to update program index for `{:?}`: Several files have occupied indexes: `{:?}`", path, intersected));
            }
            // COLLISION CHECK END

            rwlock_index.index().extend(virtual_index);
            storage.add(uuid.clone(), package as Box<dyn DedutyPackage>).await;

            Ok(uuid.to_string())
        },
        // UNKNOWN PACKAGE
        None => Err(format!("Manifest '{}' version is not supported", manifest.as_string()))
    }
}

#[tauri::command]
pub async fn getLocalPackage<'s>(storage: StateActiveStorage<'s>, id: &str) -> Result<Option<SerdeDedutyPackage>, String> {
    match storage.get(&id.to_string()).await {
        Some(package) => {
            match *package.read().await {
                ActivePackage::Online(ref real) => Ok(
                    Some(
                        SerdeDedutyPackage::try_from(real.as_ref())
                            .await
                            .map_err(|error| format!("Internal error: {}", error.to_string()))?
                    )
                ),
                ActivePackage::Offline => Err(format!("Internal error: Package '{}' is not available", id))
            }
        }
        None => Err(format!("Internal error: Package with uuid '{}' not found", id))
    }
}

#[tauri::command]
pub async fn listLocalPackage<'s>(storage: StateActiveStorage<'s>) -> Result<Vec<String>, String> {
    Ok(
        storage
            .list()
            .await
            .iter()
            .map(|uuid| uuid.to_string())
            .collect()
    )
}

#[tauri::command]
pub async fn listPackageLections<'s>(storage: StateActiveStorage<'s>, package: &str) -> Result<Vec<String>, String> {
    match storage.get(&package.to_string()).await {
        Some(active) => {
            match *active.read().await {
                ActivePackage::Online(ref real) => Ok(
                    real
                        .lections()
                        .iter()
                        .map(|lection| lection.id().to_string()).collect()),
                ActivePackage::Offline => Err(format!("Internal error: Package with ID `{}` is not available", package))
            }
        },
        None => Err(format!("Internal error: Package with ID `{}` is not exist", package))
    }
}

#[tauri::command]
pub async fn getPackageLection<'s>(storage: StateActiveStorage<'s>, package: &str, lection: &str) -> Result<SerdeDedutyLection, String> {
        match storage.get(&package.to_string()).await {
            Some(active) => {
                match *active.read().await {
                    ActivePackage::Online(ref real_package) => {
                        let real_lections = real_package.lections();
                        let real_lection = real_lections.iter()
                            .find(|real_lection| real_lection.id().as_str() == lection)
                            .ok_or_else(|| format!("Internal error: Lection with id `{}` is not exist", lection))?;

                        SerdeDedutyLection::try_from(*real_lection)
                            .await
                            .map_err(|error| format!("Internal error: While serialize lection object: {}", error.to_string()))
                    }
                    ActivePackage::Offline => Err(format!("Internal error: Package with ID `{}` is not available", package))
                }
            },
            None => Err(format!("Internal error: Package with ID `{}` is not exist", package))
        }
}

#[tauri::command]
pub async fn openFileChunked<'s>(storage: StateActiveStorage<'s>, package: StateFilePackageIndex<'s>, index: StateFileReaderIndex<'s>, id: &str) -> Result<String, String> {
    // Find out which package and / or lection\
    let file_id = id.to_string();
    match package.write().await.index().get(&file_id) {

        // CREATE TOKEN ON PACKAGE FILE
        Some(FileAffinity::Package { package: package_uuid}) => {
            match storage.get(package_uuid).await {
                Some(active) => {
                    match *active.read().await {

                        // GETTING PACKAGE
                        ActivePackage::Online(ref real) => {
                            match real.files().file(&file_id).await {
                                Ok(Some(file)) => {

                                    // GETTING READER
                                    match file.load().await {
                                        Ok(reader) => {
                                            let token = Uuid::new_v4();
                                            index.write().await.index().insert(token.to_string(), reader);
                                            Ok(token.to_string())
                                        }
                                        Err(error) => Err(format!("Internal error: {}", error.to_string()))
                                    }
                                },
                                Ok(None) => Err(format!("Internal error: Unable to find file with id `{}`", file_id)),
                                Err(error) => Err(format!("Internal error: {}", error.to_string()))
                            }
                        }
                        ActivePackage::Offline => Err(format!("Internal error: Package with ID `{}` is not available", package_uuid))
                    }
                }
                None => Err(format!("Internal error: Package with ID `{}` is not exist", package_uuid))
            }
        },

        // CREATE TOKEN ON LECTION FILE
        Some(FileAffinity::Lection { package: package_uuid,  lection: lection_uuid}) => {
            match storage.get(package_uuid).await {
                Some(active) => {
                    match *active.read().await {

                        // GETTING PACKAGE
                        ActivePackage::Online(ref real) => {
                            let lections = real.lections();
                            let lection = lections
                                .iter()
                                .find(|lection| lection.id() == *lection_uuid)
                                .ok_or_else(|| format!("Internal error: Lection with id `{}` is not exist", lection_uuid))?;

                            // GETTING READER
                            match lection.files().file(&file_id).await {
                                Ok(Some(file)) => {

                                    // GETTING READER
                                    match file.load().await {
                                        Ok(reader) => {
                                            let token = Uuid::new_v4();
                                            index.write().await.index().insert(token.to_string(), reader);
                                            Ok(token.to_string())
                                        }
                                        Err(error) => Err(format!("Internal error: {}", error.to_string()))
                                    }
                                },
                                Ok(None) => Err(format!("Internal error: Unable to find file with id `{}`", file_id)),
                                Err(error) => Err(format!("Internal error: {}", error.to_string()))
                            }
                        }
                        ActivePackage::Offline => Err(format!("Internal error: Package with ID `{}` is not available", package_uuid))
                    }
                }
                None => Err(format!("Internal error: Package with ID `{}` is not exist", package_uuid))
            }
        },
        None => Err(format!("File with id `{}` is not in program index", file_id))
    }
}

#[tauri::command]
pub async fn closeFileChunked<'s>(index: StateFileReaderIndex<'s>, id: &str) -> Result<bool, String> {
    Ok(
        index
            .write()
            .await
            .index()
            .remove(&id.to_string())
            .is_some()
    )
}

#[tauri::command]
pub async fn getFileChunked<'s>(index: StateFileReaderIndex<'s>, id: &str, chunk: usize) -> Result<Option<Vec<u8>>, String> {
    match index.write().await.index().get_mut(&id.to_string()) {
        Some(buffer) => {
            match buffer.closed() {
                true => Err(format!("File `{}` have been closed", id)),
                false => buffer
                    .next(chunk)
                    .await
                    .map_err(|err| format!("Error while reading file `{}`: {}", id, err.to_string()))
            }
        }
        None => Err(format!("File `{}` is not opened (or have been read)", id))
    }
}
