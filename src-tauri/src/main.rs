#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

use deduty::storage::ActiveStorage;

mod package;


fn main() {
    let storage = Arc::new(ActiveStorage::new());

    // TODO: Async load task 
    // storage.load()

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
          self::package::commands::addLocalPackage,
          self::package::commands::getLocalPackage,
          self::package::commands::getPackageFile,
          self::package::commands::listLocalPackage,
          self::package::commands::listPackageLections,
          self::package::commands::getPackageLection,
        ])
        .manage(storage.clone())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
    // TODO: Save storage on the disk
    // storage.save()
}
