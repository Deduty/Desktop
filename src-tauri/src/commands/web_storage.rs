use crate::managers::ServiceManager;


type StateServiceManager<'l> = tauri::State<'l, std::sync::Arc<ServiceManager>>;


#[tauri::command]
#[allow(non_snake_case)]
pub async fn webStorageExport(services: StateServiceManager<'_>, service: &str, package: &str, path: &str) -> Result<(), String> {
    println!("export! {path}");
    services
        .access(service)
        .await
        .as_web_storage()
        .map_err(|error| error.to_string())?
        .borrow()
        .export(package, path)
        .await
        .map_err(|error| error.to_string())
}


#[tauri::command]
#[allow(non_snake_case)]
pub async fn webStorageImport(services: StateServiceManager<'_>, service: &str, package: &str, path: &str) -> Result<(), String> {
    println!("import! {path}");
    services
        .access(service)
        .await
        .as_web_storage()
        .map_err(|error| error.to_string())?
        .borrow()
        .import(package, path)
        .await
        .map_err(|error| error.to_string())
}


#[tauri::command]
#[allow(non_snake_case)]
pub async fn webStorageClear(services: StateServiceManager<'_>, service: &str, package: &str) -> Result<(), String> {
    services
        .access(service)
        .await
        .as_web_storage()
        .map_err(|error| error.to_string())?
        .borrow()
        .clear(package)
        .await
        .map_err(|error| error.to_string())
}


#[tauri::command]
#[allow(non_snake_case)]
pub async fn webStorageDelete(
    services: StateServiceManager<'_>,
    service: &str,
    package: &str,
    lection: Option<&str>,
    key: &str
) -> Result<Option<String>, String> {

    services
        .access(service)
        .await
        .as_web_storage()
        .map_err(|error| error.to_string())?
        .borrow()
        .delete(package, lection, key)
        .await
        .map_err(|error| error.to_string())
}


#[tauri::command]
#[allow(non_snake_case)]
pub async fn webStorageGet(
    services: StateServiceManager<'_>,
    service: &str,
    package: &str,
    lection: Option<&str>,
    key: &str,
    fallback: Option<&str>
) -> Result<Option<String>, String> {

    services
        .access(service)
        .await
        .as_web_storage()
        .map_err(|error| error.to_string())?
        .borrow()
        .get(package, lection, key, fallback)
        .await
        .map_err(|error| error.to_string())
}


#[tauri::command]
#[allow(non_snake_case)]
pub async fn webStorageSet(
    services: StateServiceManager<'_>,
    service: &str,
    package: &str,
    lection: Option<&str>,
    key: &str,
    value: &str,
    replaced: bool
) -> Result<Option<String>, String> {
    
    services
        .access(service)
        .await
        .as_web_storage()
        .map_err(|error| error.to_string())?
        .borrow()
        .set(package, lection, key, value, replaced)
        .await
        .map_err(|error| error.to_string())
}
