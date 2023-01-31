use std::sync::Arc;

use async_std::sync::RwLock;

use deduty_package_index::{DedutyPackageIndex, PackageStatus};
use deduty_package_storage::DedutyPackageStorageIndex;
use deduty_package_traits::DedutyPackage;

use xresult::XResult;

type StatePackageIndex<'l> = tauri::State<'l, Arc<RwLock<DedutyPackageIndex>>>;
type StatePackageStorage<'l> = tauri::State<'l, Arc<RwLock<DedutyPackageStorageIndex>>>;


#[tauri::command]
#[allow(non_snake_case)]
pub async fn packageStorageDelete<'s>(packages: StatePackageIndex<'s>, storages: StatePackageStorage<'s>, package: &str, key: &str) -> Result<bool, String> {   
    let package_id = package.to_string();

    for service in packages.read().await.services_ref().values() {
        let optional_agent =
            service
                .get(&package_id)
                .await
                .map_err(|error| format!("Internal error: {}", error.to_string()))?;

        if let Some(agent) = optional_agent {
            return Ok(
                storages
                    .write()
                    .await
                    .storage(
                        Into::<XResult<&dyn DedutyPackage>>::into(
                            agent
                            .read()
                            .await
                            .package_ref())
                        .map_err(|error| format!("Internal error: {}", error.to_string()))?)
                    .await
                    .map_err(|error| format!("Internal error: {}", error.to_string()))?
                    .write()
                    .await
                    .package_mut()
                    .remove(&key.to_string())
                    .is_some());
        }
    }

    Err(format!("Internal error: Package with uuid '{}' not found", package))
}

#[tauri::command]
#[allow(non_snake_case)]
pub async fn packageStorageGet<'s>(packages: StatePackageIndex<'s>, storages: StatePackageStorage<'s>, package: &str, key: &str) -> Result<Option<String>, String> {   
    let package_id = package.to_string();

    for service in packages.read().await.services_ref().values() {
        let optional_agent =
            service
                .get(&package_id)
                .await
                .map_err(|error| format!("Internal error: {}", error.to_string()))?;

        if let Some(agent) = optional_agent {
            return Ok(
                storages
                    .write()
                    .await
                    .storage(
                        Into::<XResult<&dyn DedutyPackage>>::into(
                            agent
                            .read()
                            .await
                            .package_ref())
                        .map_err(|error| format!("Internal error: {}", error.to_string()))?)
                    .await
                    .map_err(|error| format!("Internal error: {}", error.to_string()))?
                    .write()
                    .await
                    .package_mut()
                    .get(&key.to_string())
                    .cloned());
        }
    }

    Err(format!("Internal error: Package with uuid '{}' not found", package))
}

#[tauri::command]
#[allow(non_snake_case)]
pub async fn packageStorageSet<'s>(packages: StatePackageIndex<'s>, storages: StatePackageStorage<'s>, package: &str, key: &str, value: &str, replace: bool) -> Result<bool, String> {   
    let package_id = package.to_string();

    for service in packages.read().await.services_ref().values() {
        let optional_agent =
            service
                .get(&package_id)
                .await
                .map_err(|error| format!("Internal error: {}", error.to_string()))?;

        if let Some(agent) = optional_agent {
            let mut insertion = true;

            storages
                .write()
                .await
                .storage(
                    Into::<XResult<&dyn DedutyPackage>>::into(
                        agent
                        .read()
                        .await
                        .package_ref())
                    .map_err(|error| format!("Internal error: {}", error.to_string()))?)
                .await
                .map_err(|error| format!("Internal error: {}", error.to_string()))?
                .write()
                .await
                .package_mut()
                .entry(key.to_string())
                .and_modify(|stored| match replace {
                    true => *stored = value.to_string(),
                    false => insertion = false
                })
                .or_insert(value.to_string());

            return Ok(insertion);
        }
    }

    Err(format!("Internal error: Package with uuid '{}' not found", package))
}

#[tauri::command]
#[allow(non_snake_case)]
pub async fn lectionStorageDelete<'s>(packages: StatePackageIndex<'s>, storages: StatePackageStorage<'s>, package: &str, lection: &str, key: &str) -> Result<bool, String> {   
    let package_id = package.to_string();
    let lection_id = lection.to_string();

    for service in packages.read().await.services_ref().values() {
        let optional_agent =
            service
                .get(&package_id)
                .await
                .map_err(|error| format!("Internal error: {}", error.to_string()))?;

        if let Some(agent) = optional_agent {
            // LECTION EXISTENCE CHECK
            match agent.read().await.package_ref() {
                PackageStatus::Online(package) =>
                    if package.lections().iter().find(|lection| lection.id() == lection_id).is_none() {
                        return Err(format!("Internal error: Lection with uuid '{}' not found", lection))
                    },
                PackageStatus::Offline => return Err(format!("Internal error: Lection with uuid '{}' not available", lection)),
            }

            return Ok(
                storages
                    .write()
                    .await
                    .storage(
                        Into::<XResult<&dyn DedutyPackage>>::into(
                            agent
                            .read()
                            .await
                            .package_ref())
                        .map_err(|error| format!("Internal error: {}", error.to_string()))?)
                    .await
                    .map_err(|error| format!("Internal error: {}", error.to_string()))?
                    .write()
                    .await
                    .lections_mut()
                    .entry(lection_id)
                    .or_default()
                    .remove(&key.to_string())
                    .is_some());
        }
    }

    Err(format!("Internal error: Package with uuid '{}' not found", package))
}

#[tauri::command]
#[allow(non_snake_case)]
pub async fn lectionStorageGet<'s>(packages: StatePackageIndex<'s>, storages: StatePackageStorage<'s>, package: &str, lection: &str, key: &str) -> Result<Option<String>, String> {   
    let package_id = package.to_string();
    let lection_id = lection.to_string();

    for service in packages.read().await.services_ref().values() {
        let optional_agent =
            service
                .get(&package_id)
                .await
                .map_err(|error| format!("Internal error: {}", error.to_string()))?;

        if let Some(agent) = optional_agent {
            // LECTION EXISTENCE CHECK
            match agent.read().await.package_ref() {
                PackageStatus::Online(package) =>
                    if package.lections().iter().find(|lection| lection.id() == lection_id).is_none() {
                        return Err(format!("Internal error: Lection with uuid '{}' not found", lection))
                    },
                PackageStatus::Offline => return Err(format!("Internal error: Lection with uuid '{}' not available", lection)),
            }

            return Ok(
                storages
                    .write()
                    .await
                    .storage(
                        Into::<XResult<&dyn DedutyPackage>>::into(
                            agent
                            .read()
                            .await
                            .package_ref())
                        .map_err(|error| format!("Internal error: {}", error.to_string()))?)
                    .await
                    .map_err(|error| format!("Internal error: {}", error.to_string()))?
                    .write()
                    .await
                    .lections_mut()
                    .entry(lection_id)
                    .or_default()
                    .get(&key.to_string())
                    .cloned());
        }
    }

    Err(format!("Internal error: Package with uuid '{}' not found", package))
}

#[tauri::command]
#[allow(non_snake_case)]
pub async fn lectionStorageSet<'s>(packages: StatePackageIndex<'s>, storages: StatePackageStorage<'s>, package: &str, lection: &str, key: &str, value: &str, replace: bool) -> Result<bool, String> {   
    let package_id = package.to_string();
    let lection_id = lection.to_string();

    for service in packages.read().await.services_ref().values() {
        let optional_agent =
            service
                .get(&package_id)
                .await
                .map_err(|error| format!("Internal error: {}", error.to_string()))?;

        if let Some(agent) = optional_agent {
            // LECTION EXISTENCE CHECK
            match agent.read().await.package_ref() {
                PackageStatus::Online(package) =>
                    if package.lections().iter().find(|lection| lection.id() == lection_id).is_none() {
                        return Err(format!("Internal error: Lection with uuid '{}' not found", lection))
                    },
                PackageStatus::Offline => return Err(format!("Internal error: Lection with uuid '{}' not available", lection)),
            }

            let mut insertion = true;

            storages
                .write()
                .await
                .storage(
                    Into::<XResult<&dyn DedutyPackage>>::into(
                        agent
                        .read()
                        .await
                        .package_ref())
                    .map_err(|error| format!("Internal error: {}", error.to_string()))?)
                .await
                .map_err(|error| format!("Internal error: {}", error.to_string()))?
                .write()
                .await
                .lections_mut()
                .entry(lection_id)
                .or_default()
                .entry(key.to_string())
                .and_modify(|stored| match replace {
                    true => *stored = value.to_string(),
                    false => insertion = false
                })
                .or_insert(value.to_string());
            
            return Ok(insertion);
        }
    }

    Err(format!("Internal error: Package with uuid '{}' not found", package))
}
