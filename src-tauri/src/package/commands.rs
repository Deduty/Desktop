use std::str::FromStr;
use std::sync::Arc;

use async_std::io::ReadExt;
use async_std::fs::File;
use async_std::path::{ Path, PathBuf };

use deduty::package::package::traits::DedutyPackage;
use deduty::package::package::serde::DedutyPackage as SerDedutyPackage;

use deduty::storage::ActiveStorage;
use deduty::storage::active::ActivePackage;

type StateStorage<'l> = tauri::State<'l, Arc<ActiveStorage>>;


#[tauri::command]
pub async fn addLocalPackage<'s>(storage: StateStorage<'s>, path: &str) -> Result<String, String> {
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
            let package = premier::actions::load(path.into())
                .await
                .map_err(|error| format!("Internal error: {}", error.to_string()))?;
            let uuid = package.id().clone();

            storage.add(uuid.clone(), package as Box<dyn DedutyPackage>).await;

            Ok(uuid.to_string())
        },
        // UNKNOWN PACKAGE
        None => Err(format!("Manifest '{}' version is not supported", manifest.as_string()))
    }
}


#[tauri::command]
pub async fn getLocalPackage<'s>(storage: StateStorage<'s>, id: &str) -> Result<Option<SerDedutyPackage>, String> {
    let uuid = uuid::Uuid::parse_str(id)
        .map_err(|error| format!("Internal error: {}", error.to_string()))?;

    match storage.get(&uuid).await {
        Some(package) => {
            match *package.read().await {
                ActivePackage::Online(ref real) => Ok(
                    Some(
                        SerDedutyPackage::try_from(real.as_ref())
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
pub async fn listLocalPackage<'s>(storage: StateStorage<'s>) -> Result<Vec<String>, String> {
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
pub async fn getPackageFile<'s>(storage: StateStorage<'s>, id: &str, location: &str) -> Result<String, String> {
    let uuid = uuid::Uuid::parse_str(id)
        .map_err(|error| error.to_string())?;

    let path = PathBuf::from_str(location)
        .map_err(|_| format!("Internal error: Invalid file location: '{location}'"))?;

    let mut content = String::new();

    match storage.get(&uuid).await {
        Some(package) => {
            match *package.read().await {
                ActivePackage::Online(ref real) => {
                    let maybe_file = real
                        .as_ref()
                        .files()
                        .file(path.as_path())
                        .await
                        .map_err(|error| error.to_string())?;

                    match maybe_file {
                        Some(file) => {
                            file.load()
                                .await
                                .map_err(|error| format!("Internal error: While loading file, getting this error: {}", error.to_string()))?
                                .read_to_string(&mut content)
                                .await
                                .map_err(|error| format!("Internal error: While loading file, getting this error: {}", error.to_string()))?;
                            Ok(content)
                        },
                        None => Err(format!("Internal error: Package file not found at location '{location}'"))
                    }
                },
                ActivePackage::Offline => Err(format!("Internal error: Package '{}' is not available", id))
            }
        },
        None => Err(format!("Internal error: Package with uuid '{}' not found", id))
    }
}
