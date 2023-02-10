use xresult::XError;

use deduty_service::{ AddService, UpdateService, Service };

use crate::managers::ServiceManager;

type StateServiceManager<'l> = tauri::State<'l, std::sync::Arc<ServiceManager>>;


#[tauri::command]
#[allow(non_snake_case)]
pub async fn getServiceAddRequirements(services: StateServiceManager<'_>, service: &str) -> Result<String, String> {
    let service = services
        .get(service)
        .await
        .ok_or_else(|| XError::from(("Internal error", format!("Service with id `{service}` not found"))).to_string())?;
    
        Ok(<dyn Service as AddService>::requirements(service.as_ref()).clone())
}


#[tauri::command]
#[allow(non_snake_case)]
pub async fn getServiceUpdateRequirements(services: StateServiceManager<'_>, service: &str) -> Result<String, String> {
    let service = services
        .get(service)
        .await
        .ok_or_else(|| XError::from(("Internal error", format!("Service with id `{service}` not found"))).to_string())?;
    
    Ok(<dyn Service as UpdateService>::requirements(service.as_ref()).clone())
}


#[tauri::command]
#[allow(non_snake_case)]
pub async fn listServices(services: StateServiceManager<'_>) -> Result<Vec<String>, String> {
    Ok(
        services
            .list()
            .await
            .map(|service| service.id().clone())
            .collect()
    )
}
