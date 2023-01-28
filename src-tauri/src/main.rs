#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::str::FromStr;
use std::sync::Arc;

use async_std::path::PathBuf;
use async_std::sync::RwLock;

use deduty_package_index::DedutyPackageIndex;
use deduty_package_storage::DedutyPackageStorageIndex;
use deduty_application_resources::reader::FileReaderIndex;


mod commands;
mod manifest;


fn main() {
    let packages = Arc::new(RwLock::new(DedutyPackageIndex::new()));
    let readers = Arc::new(RwLock::new(FileReaderIndex::new()));
    let storages = Arc::new(RwLock::new(DedutyPackageStorageIndex::new(PathBuf::from_str("O:\\").expect("Invalid path"))));

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // CHUNKED
            self::commands::chunked::closeFileChunked,
            self::commands::chunked::getFileChunked,
            self::commands::chunked::openFileChunked,

            // PACKAGE
            self::commands::package::addLocalPackage,
            self::commands::package::getPackage,
            self::commands::package::getPackageLection,
            self::commands::package::listPackages,
            self::commands::package::listPackageLections,

            // STORAGE - PACKAGE
            self::commands::storage::packageStorageDelete,
            self::commands::storage::packageStorageGet,
            self::commands::storage::packageStorageSet,

            // STORAGE - LECTION
            self::commands::storage::lectionStorageDelete,
            self::commands::storage::lectionStorageGet,
            self::commands::storage::lectionStorageSet,
        ])
        .manage(packages.clone())
        .manage(readers.clone())
        .manage(storages.clone())
        .build(tauri::generate_context!())
        .expect("Error while running tauri application")
        .run(move |_handle, event| {
            let storages = storages.clone();

            match event {
                tauri::RunEvent::Exit => {
                    let storage_save_thread = std::thread::spawn(move || {
                        async_std::task::block_on(async move {
                            let results = storages
                                .read()
                                .await
                                .save()
                                .await;
    
                            for error in results.iter().filter_map(|reason| reason.as_ref().err().map(|error| error.to_string())) {
                                println!("While save error occurred {error}")
                            }
                        })
                    });
    
                    // Ignore errors
                    let _ = storage_save_thread.join();
                },
                _ => {}
            }
        });
}
