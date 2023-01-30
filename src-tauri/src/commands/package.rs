use std::sync::Arc;

use async_std::io::ReadExt;
use async_std::fs::File;
use async_std::sync::RwLock;
use async_std::path::Path;

use deduty_package_traits::DedutyPackage;
use deduty_package_serde::{
    SerdeDedutyPackage,
    SerdeDedutyLection
};

use deduty_package_index::DedutyPackageIndex;

use xresult::XResult;

use crate::manifest::{
    PackageManifest,
    PackageManifestVariants
};

type StatePackageIndex<'l> = tauri::State<'l, Arc<RwLock<DedutyPackageIndex>>>;


#[tauri::command]
pub async fn addLocalPackage<'s>(packages: StatePackageIndex<'s>, path: &str) -> Result<String, String> {
    // PATH
	let target = Path::new(path);
    if !target.exists().await {
        return Err(format!("Path '{:#?}' is not exist", path));
    }
  	if !target.is_dir().await {
    	return Err(format!("Path '{:#?}' is not a directory", path));
  	}

    // PACKAGE
    let package_toml = target.join("package.toml");
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
    let manifest: PackageManifest = 
        toml::from_slice(&package_toml_content)
            .map_err(|error| format!("Internal error: {}", error.to_string()))?;

    match manifest.to_enum() {
        // PREMIER PACKAGE
        Some(PackageManifestVariants::Premier) => Ok(
            Into::<XResult<&dyn DedutyPackage>>::into(
                // Creating a new package
                packages
                    .write()
                    .await
                    .services_mut()
                    .get_mut(&PackageManifestVariants::Premier.to_string())
                    .ok_or("Internal error: Premier package service is offline".to_string())?
                    .add(path.to_string())
                    .await
                    .map_err(|error| format!("Internal error: {}", error.to_string()))?
                    .read()
                    .await
                    .package_ref())

                // XResult for &dyn DedutyPackage
                .map_err(|error| format!("Internal error: {}", error.to_string()))?
                .id()
        ),
        // UNKNOWN PACKAGE
        None => Err(format!("Manifest '{}' version is not supported", manifest.as_string()))
    }
}

#[tauri::command]
pub async fn getPackage<'s>(packages: StatePackageIndex<'s>, id: &str) -> Result<SerdeDedutyPackage, String> {
    let package_id = id.to_string();

    for service in packages.read().await.services_ref().values() {
        match service.get(&package_id).await.map_err(|error| format!("Internal error: {}", error.to_string()))? {
            Some(agent) =>
                return SerdeDedutyPackage::try_from(
                    Into::<XResult<&dyn DedutyPackage>>::into(
                        agent
                            .read()
                            .await
                            .package_ref())
                        .map_err(|error| format!("Internal error: {}", error.to_string()))?)
                    .await
                    .map_err(|error| format!("Internal error: While serialize package object: {}", error.to_string())),
            None => {}
        }
    }

    Err(format!("Internal error: Package with uuid '{}' not found", id))
}

#[tauri::command]
pub async fn listPackages<'s>(packages: StatePackageIndex<'s>) -> Result<Vec<String>, String> {
    let mut hierarchy = vec![];

    for service in packages.read().await.services_ref().values() {
        hierarchy.push(
            service
                .list()
                .await
                .map_err(|error| format!("Internal error: {}", error.to_string()))?)
    }

    Ok(hierarchy.into_iter().flatten().collect())
}

#[tauri::command]
pub async fn listPackageLections<'s>(packages: StatePackageIndex<'s>, id: &str) -> Result<Vec<String>, String> {
    let package_id = id.to_string();

    for service in packages.read().await.services_ref().values() {
        match service.get(&package_id).await.map_err(|error| format!("Internal error: {}", error.to_string()))? {
            Some(agent) => return Ok(
                Into::<XResult<&dyn DedutyPackage>>::into(
                    agent
                        .read()
                        .await
                        .package_ref())
                .map_err(|error| format!("Internal error: {}", error.to_string()))?
                .lections()
                .into_iter()
                .map(|lection| lection.id())
                .collect()
            ),
            None => {}
        }
    }

    Err(format!("Internal error: Package with uuid '{}' not found", id))
}

#[tauri::command]
pub async fn getPackageLection<'s>(packages: StatePackageIndex<'s>, package: &str, lection: &str) -> Result<SerdeDedutyLection, String> {
    let package_id = package.to_string();
    let lection_id = lection.to_string();

    for service in packages.read().await.services_ref().values() {
        match service.get(&package_id).await.map_err(|error| format!("Internal error: {}", error.to_string()))? {
            Some(agent) =>
                return SerdeDedutyLection::try_from(
                    Into::<XResult<&dyn DedutyPackage>>::into(
                        agent
                            .read()
                            .await
                            .package_ref())
                    .map_err(|error| format!("Internal error: {}", error.to_string()))?
                    .lections()
                    .into_iter()
                    .find(|lection| lection.id() == lection_id)
                    .ok_or_else(|| format!("Internal error: Package with ID `{}` is not found", package))?)
                .await
                .map_err(|error| format!("Internal error: While serialize package object: {}", error.to_string())),
            None => {}
        }
    }

    Err(format!("Internal error: Package with uuid '{}' not found", package))
}
