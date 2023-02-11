use xresult::XError;

use deduty_package::{
    SerdePackage,
    serde::AsyncTrySerialize,
    serde::package::DedutyPackageSerde
};

use crate::managers::ServiceManager;

type StateServiceManager<'l> = tauri::State<'l, std::sync::Arc<ServiceManager>>;


#[tauri::command]
#[allow(non_snake_case)]
pub async fn addPackage(services: StateServiceManager<'_>, service: &str, serialized: &str) -> Result<String, String> {
    services
        .get(service)
        .await
        .ok_or_else(|| XError::from(("Internal error", format!("Service with id `{service}` not found"))).to_string())?
        .add(serialized)
        .await
        .map(|package| package.id().to_string())
        .map_err(|error| error.to_string())
}


#[tauri::command]
#[allow(non_snake_case)]
pub async fn getPackage(services: StateServiceManager<'_>, service: &str, package: &str) -> Result<DedutyPackageSerde, String> {
    let entry = services.access(service).await;
    let package = entry.with_package(package).await.map_err(|error| error.to_string())?;

    (package as &dyn SerdePackage)
        .try_serde()
        .await
        .map_err(|error| error.to_string())
}


#[tauri::command]
#[allow(non_snake_case)]
pub async fn listPackages(services: StateServiceManager<'_>, service: &str) -> Result<Vec<String>, String> {
    Ok(
        services
            .get(service)
            .await
            .ok_or_else(|| XError::from(("Internal error", format!("Service with id `{service}` not found"))).to_string())?
            .all()
            .await
            .map_err(|error| error.to_string())?
            .map(|package| package.id().to_string())
            .collect()
    )
}


#[tauri::command]
#[allow(non_snake_case)]
pub async fn subPackage(services: StateServiceManager<'_>, service: &str, package: &str) -> Result<(), String> {
    services
        .get(service)
        .await
        .ok_or_else(|| XError::from(("Internal error", format!("Service with id `{service}` not found"))).to_string())?
        .sub(package)
        .await
        .map_err(|error| error.to_string())
}


#[tauri::command]
#[allow(non_snake_case)]
pub async fn updatePackage(services: StateServiceManager<'_>, service: &str, package: &str, serialized: &str) -> Result<(), String> {
    services
        .get(service)
        .await
        .ok_or_else(|| XError::from(("Internal error", format!("Service with id `{service}` not found"))).to_string())?
        .update(package, serialized)
        .await
        .map_err(|error| error.to_string())
}
