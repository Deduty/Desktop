use std::sync::Arc;

use async_std::sync::RwLock;
use uuid::Uuid;

use deduty_application_resources::reader::FileReaderIndex;
use deduty_package_index::DedutyPackageIndex;
use deduty_package_traits::DedutyPackage;
use xresult::XResult;


type StatePackageIndex<'l> = tauri::State<'l, Arc<RwLock<DedutyPackageIndex>>>;
type StateReaderIndex<'l> = tauri::State<'l, Arc<RwLock<FileReaderIndex>>>;


#[tauri::command]
pub async fn openFileChunked<'s>(packages: StatePackageIndex<'s>, readers: StateReaderIndex<'s>, package: &str, id: &str) -> Result<String, String> {
    let package_id = package.to_string();
    let file_id = id.to_string();

    for service in packages.read().await.services_ref().values() {
        let optional_agent =
            service
                .get(&package_id)
                .await
                .map_err(|error| format!("Internal error: {}", error.to_string()))?;

        if let Some(agent) = optional_agent {
            // SEARCH IN PACKAGE
            // Note: Such long `if let` expression required for RwLockReadGuard of agent
            //
            if let Some(file) = 
                Into::<XResult<&dyn DedutyPackage>>::into(
                    agent
                        .read()
                        .await
                        .package_ref())
                    .map_err(|error| format!("Internal error: {}", error.to_string()))?
                    .files()
                    .file(&file_id)
                    .await
                    .map_err(|error| format!("Internal error: {}", error.to_string()))?
            {
                match file.load().await {
                    Ok(reader) => {
                        let token = Uuid::new_v4();

                        readers
                            .write()
                            .await
                            .index()
                            .insert(token.to_string(), reader);

                        return Ok(token.to_string())
                    }
                    Err(error) => return Err(format!("Internal error: {}", error.to_string()))
                }
            }

            // SEARCH IN LECTIONS
            // Note: Such long `for in` expression required for RwLockReadGuard of agent
            //
            for lection in
                Into::<XResult<&dyn DedutyPackage>>::into(
                    agent
                        .read()
                        .await
                        .package_ref())
                    .map_err(|error| format!("Internal error: {}", error.to_string()))?
                    .lections()
            {
                let optional_file =
                    lection
                        .files()
                        .file(&file_id)
                        .await
                        .map_err(|error| format!("Internal error: {}", error.to_string()))?;

                if let Some(file) = optional_file {
                    match file.load().await {
                        Ok(reader) => {
                            let token = Uuid::new_v4();
    
                            readers
                                .write()
                                .await
                                .index()
                                .insert(token.to_string(), reader);
    
                            return Ok(token.to_string())
                        }
                        Err(error) => return Err(format!("Internal error: {}", error.to_string()))
                    }
                }
            }
        }
    }

    Err(format!("Internal error: Package with uuid '{}' not found", id))
}

#[tauri::command]
pub async fn closeFileChunked<'s>(readers: StateReaderIndex<'s>, id: &str) -> Result<bool, String> {
    Ok(
        readers
            .write()
            .await
            .index()
            .remove(&id.to_string())
            .is_some()
    )
}

#[tauri::command]
pub async fn getFileChunked<'s>(readers: StateReaderIndex<'s>, id: &str, chunk: usize) -> Result<Option<Vec<u8>>, String> {
    match readers.write().await.index().get_mut(&id.to_string()) {
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
