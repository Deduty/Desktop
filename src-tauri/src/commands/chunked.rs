use std::sync::Arc;

use xresult::XError;

use crate::managers::{ ReaderManager, ServiceManager };

type StateReaderManager<'l> = tauri::State<'l, Arc<ReaderManager>>;
type StateServiceManager<'l> = tauri::State<'l, Arc<ServiceManager>>;


#[tauri::command]
#[allow(non_snake_case)]
pub async fn closeFileChunked(readers: StateReaderManager<'_>, file: &str) -> Result<(), String> {
    if let Some(reader) = readers.sub(file).await {
        return reader.close().await.map_err(|error| error.to_string());
    }

    Err(XError::from(("Internal error", format!("Reader with id `{file}` not found"))).to_string())
}


#[tauri::command]
#[allow(non_snake_case)]
pub async fn openFileChunked(
    services: StateServiceManager<'_>,
    readers: StateReaderManager<'_>,
    service: &str,
    package: &str,
    lection: &str,
    file: &str
) -> Result<String, String> {
    let reader = services
        .access(service)
        .await
        .with_file(package, lection, file)
        .await
        .map_err(|error| error.to_string())?
        .reader()
        .await
        .map_err(|error| error.to_string())?;

    Ok(readers.add(reader).await)
}


#[tauri::command]
#[allow(non_snake_case)]
pub async fn getFileChunked<'s>(readers: StateReaderManager<'s>, file: &str, chunk: usize) -> Result<Option<Vec<u8>>, String> {
    readers
        .get(file)
        .await
        .ok_or_else(|| XError::from(("Internal error", format!("Reader with id `{file}` not found"))).to_string())?
        .next(chunk)
        .await
        .map_err(|error| error.to_string())
}
