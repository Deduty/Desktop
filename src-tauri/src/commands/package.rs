use std::sync::Arc;
use std::collections::HashMap;

use async_std::sync::RwLock;

use deduty_package_serde::{
    SerdeDedutyPackage,
    SerdeDedutyLection
};

use deduty_package_index::{
    DedutyPackageIndex,
    FrontEndSerialization
};
use deduty_package_storage::DedutyPackageStorageIndex;

type StatePackageIndex<'l> = tauri::State<'l, Arc<RwLock<DedutyPackageIndex>>>;
type StatePackageStorage<'l> = tauri::State<'l, Arc<RwLock<DedutyPackageStorageIndex>>>;


#[tauri::command]
#[allow(non_snake_case)]
pub async fn getService(packages: StatePackageIndex<'_>, package: &str) -> Result<String, String> {
    let package_id = package.to_string();

    for (key, service) in packages.read().await.services_ref() {
        if
            service
                .has(&package_id)
                .await
                .map_err(|error| format!("Internal error: Unable to update package due to unexpected error: {error}"))?
        {
            return Ok(key.clone());
        }
    }

    Err(format!("Internal error: Service not found for package with id `{package_id}`"))
}

#[tauri::command]
#[allow(non_snake_case)]
pub async fn listServiceAddRequirements(packages: StatePackageIndex<'_>) -> Result<HashMap<String, HashMap<String, String>>, String> {
    let mut reqs = HashMap::with_capacity(packages.read().await.services_ref().len());
    for (name, service) in packages.read().await.services_ref() {
        reqs.insert(
            name.clone(),
            service
                .add_reqs()
                .iter()
                .map(|(key, req)| (key.clone(), req.to_string()))
                .collect()
        );
    }
    Ok(reqs)
}

#[tauri::command]
#[allow(non_snake_case)]
pub async fn listServiceUpdateRequirements(packages: StatePackageIndex<'_>) -> Result<HashMap<String, HashMap<String, String>>, String> {
    let mut reqs = HashMap::with_capacity(packages.read().await.services_ref().len());
    for (name, service) in packages.read().await.services_ref() {
        reqs.insert(
            name.clone(),
            service
                .update_reqs()
                .iter()
                .map(|(key, req)| (key.clone(), req.to_string()))
                .collect()
        );
    }
    Ok(reqs)
}

#[tauri::command]
#[allow(non_snake_case)]
pub async fn addPackage<'s>(packages: StatePackageIndex<'s>, service: &str, mut serialized: HashMap<String, String>) -> Result<String, String> {
    let service_name = service.to_string();

    match packages
        .write()
        .await
        .services_mut()
        .get_mut(&service_name)
    {
        Some(service) => {
            let mut reserialized: HashMap<String, FrontEndSerialization> = HashMap::with_capacity(service.add_reqs().len());

            for (key, req) in service.add_reqs() {
                // Note: When serialized is large then it needs no error occurs
                match serialized.remove(key) {
                    Some(value) => {
                        reserialized.insert(key.clone(), FrontEndSerialization::new(*req, value));
                    },
                    None => return Err(format!("Internal error: Incomplete serialization for service `{service_name}`: Key `{key}` is missed"))
                }
            }

            Ok(
                service
                    .add(reserialized)
                    .await
                    .map_err(|error| format!("Internal error: Unable to add package due to unexpected error: {error}"))?
                    .read()
                    .await
                    .package_ref()
                    .to_result()
                    .map_err(|error| format!("Internal error: Unable to add package due to unexpected error: {error}"))?
                    .id()
            )
        },
        None => Err(format!("Internal error: Service with name {service} not found"))
    }
}

#[tauri::command]
#[allow(non_snake_case)]
pub async fn updatePackage<'s>(packages: StatePackageIndex<'s>, service: &str, id: &str, mut serialized: HashMap<String, String>) -> Result<(), String> {
    let service_name = service.to_string();
    let package_id = id.to_string();

    match packages
        .write()
        .await
        .services_mut()
        .get_mut(&service_name)
    {
        Some(service) => {
            // Note: Without this precheck all preparing process may going for nothing
            if !service
                    .has(&package_id)
                    .await
                    .map_err(|error|
                        format!("Internal error: Unable to check package existence for id `{package_id}` at service `{service_name}` due to unexpected error: {error}"))?
            {
                return Err(format!("Internal error: No package with id `{package_id}` found at service `{service_name}`"));
            }

            let mut reserialized: HashMap<String, FrontEndSerialization> = HashMap::with_capacity(service.add_reqs().len());

            for (key, req) in service.update_reqs() {
                // Note: When serialized is large then it needs no error occurs
                match serialized.remove(key) {
                    Some(value) => {
                        reserialized.insert(key.clone(), FrontEndSerialization::new(*req, value));
                    },
                    None => return Err(format!("Internal error: Incomplete serialization for service `{service_name}`: Key `{key}` is missed"))
                }
            }

            Ok(
                service
                    .update(reserialized, &package_id)
                    .await
                    .map_err(|error| format!("Internal error: Unable to add package due to unexpected error: {error}"))?
            )
        },
        None => Err(format!("Internal error: Service with name {service} not found"))
    }
}

#[tauri::command]
#[allow(non_snake_case)]
pub async fn getPackage<'s>(packages: StatePackageIndex<'s>, id: &str) -> Result<SerdeDedutyPackage, String> {
    let package_id = id.to_string();

    for service in packages.read().await.services_ref().values() {
        if let Some(agent) =
            service.get(&package_id).await.map_err(|error| format!("Internal error: {error}"))? {
                return SerdeDedutyPackage::try_from(
                        agent
                            .read()
                            .await
                            .package_ref()
                            .to_result()
                            .map_err(|error| format!("Internal error: {error}"))?)
                    .await
                    .map_err(|error| format!("Internal error: While serialize package object: {error}"));
        }
    }

    Err(format!("Internal error: Package with uuid '{id}' not found"))
}

#[tauri::command]
#[allow(non_snake_case)]
pub async fn subPackage<'s>(packages: StatePackageIndex<'s>, storages: StatePackageStorage<'s>, id: &str) -> Result<(), String> {
    let package_id = id.to_string();

    for service in packages.write().await.services_mut().values_mut() {
        if service
            .has(&package_id)
            .await
            .map_err(|error| format!("Internal error: {error}"))?
        {
            storages
                .write()
                .await
                .remove(&package_id)
                .await
                .map_err(|error| format!("Internal error: {error}"))?;

            service
                .sub(&package_id)
                .await
                .map_err(|error| format!("Internal error: {error}"))?;

            return Ok(());
        }
    }

    Err(format!("Internal error: Package with uuid '{id}' not found"))
}

#[tauri::command]
#[allow(non_snake_case)]
pub async fn listPackages(packages: StatePackageIndex<'_>) -> Result<Vec<String>, String> {
    let mut hierarchy = vec![];

    for service in packages.read().await.services_ref().values() {
        hierarchy.push(
            service
                .list()
                .await
                .map_err(|error| format!("Internal error: {error}"))?)
    }

    Ok(hierarchy.into_iter().flatten().collect())
}

#[tauri::command]
#[allow(non_snake_case)]
pub async fn listPackageLections<'s>(packages: StatePackageIndex<'s>, id: &str) -> Result<Vec<String>, String> {
    let package_id = id.to_string();

    for service in packages.read().await.services_ref().values() {
        if let Some(agent) =
            service.get(&package_id).await.map_err(|error| format!("Internal error: {error}"))? {
                return Ok(
                    agent
                        .read()
                        .await
                        .package_ref()
                        .to_result()
                        .map_err(|error| format!("Internal error: {error}"))?
                        .lections()
                        .into_iter()
                        .map(|lection| lection.id())
                        .collect()
                );
        }
    }

    Err(format!("Internal error: Package with uuid '{id}' not found"))
}

#[tauri::command]
#[allow(non_snake_case)]
pub async fn getPackageLection<'s>(packages: StatePackageIndex<'s>, package: &str, lection: &str) -> Result<SerdeDedutyLection, String> {
    let package_id = package.to_string();
    let lection_id = lection.to_string();

    for service in packages.read().await.services_ref().values() {
        if let Some(agent) =
            service.get(&package_id).await.map_err(|error| format!("Internal error: {error}"))? {
                return SerdeDedutyLection::try_from(
                    agent
                        .read()
                        .await
                        .package_ref()
                        .to_result()
                        .map_err(|error| format!("Internal error: {error}"))?
                        .lections()
                        .into_iter()
                        .find(|lection| lection.id() == lection_id)
                        .ok_or_else(|| format!("Internal error: Package with ID `{package}` is not found"))?)
                .await
                .map_err(|error| format!("Internal error: While serialize package object: {error}"));
        }
    }

    Err(format!("Internal error: Package with uuid `{package}` not found"))
}
