#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

use async_std::sync::RwLock;

use deduty_storage::active::ActiveStorage;

mod package;
mod index;


fn main() {
    let active_storage = Arc::new(ActiveStorage::new());
    let package_index = Arc::new(RwLock::new(index::package::FilePackageIndex::new()));
    let reader_index = Arc::new(RwLock::new(index::reader::FileReaderIndex::new()));

    // TODO: Async load (storage, then package index)

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
          self::package::commands::addLocalPackage,
          self::package::commands::getLocalPackage,
          
          self::package::commands::listLocalPackage,
          self::package::commands::listPackageLections,
          self::package::commands::getPackageLection,

          self::package::commands::closeFileChunked,
          self::package::commands::openFileChunked,
          self::package::commands::getFileChunked,
        ])
        .manage(active_storage.clone())
        .manage(package_index)
        .manage(reader_index.clone())
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
    
    // TODO: Save
    // storage.save()
}
