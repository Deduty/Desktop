#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

use async_std::sync::RwLock;

use deduty_application_settings::Settings;
use deduty_application_resources::reader::FileReaderIndex;

use deduty_package_index::{ DedutyPackageIndex, IndexService };
use deduty_package_storage::DedutyPackageStorageIndex;


mod commands;
mod manifest;


fn main() {
    // TODO: Provide logs or something for user when settings are not available
    let settings = Settings::new().unwrap();

    let packages = Arc::new(RwLock::new(DedutyPackageIndex::new()));
    let readers = Arc::new(RwLock::new(FileReaderIndex::new()));
    let storages = Arc::new(RwLock::new(DedutyPackageStorageIndex::new(settings.resources().join("storages"))));

    let service_tear_up = {
        /* Services parallel setup */
        let packages = packages.clone();
        let settings = settings.clone();

        std::thread::spawn(move || {
            async_std::task::block_on(async move {
                // Premier service
                let premier = Box::new(deduty_scheme_premier::service::PremierIndexService::new()) as Box<dyn IndexService>;
                packages.write().await.services_mut().insert("premier".to_string(), premier);
    
                // Load all
                let mut failures = Vec::new();
                for (key, service) in packages.write().await.services_mut().iter_mut() {
                    let expected_path = settings.resources().join("services").join(key);
                    match service.load_all(&expected_path).await {
                        Ok(_) => { /* TODO: Log all wrong entries */ },
                        Err(_) => {
                            // TODO: Log error
                            // Note: This service must be unplugged since this thread is not main
                            //       so we can't interrupt initialization without join this thread
                            failures.push(key.clone());
                        }
                    }
                }

                // Remove all failure services
                // Note: Irrefutable pattern required for keeping WriteRwLockGuard of packages
                //
                #[allow(irrefutable_let_patterns)]
                if let services = packages.write().await.services_mut() {
                    for failure in failures {
                        services.remove(&failure);
                    }
                }
            })
        })
    };

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
            let packages = packages.clone();
            let settings = settings.clone();

            match event {
                tauri::RunEvent::Exit => {
                    let storages_save_thread = std::thread::spawn(move || {
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

                    let packages_save_thread = std::thread::spawn(move || {
                        async_std::task::block_on(async move {
                            for (key, service) in packages.write().await.services_mut().iter_mut() {
                                let expected_path = settings.resources().join("services").join(key);
                                // TODO: Log errors
                                let _ = service.save_all(&expected_path).await;
                            }
                        })
                    });
    
                    // Ignore errors
                    let _ = storages_save_thread.join();
                    let _ = packages_save_thread.join();
                },
                _ => {}
            }
        });

    // TODO: This thread never join
    let _ = service_tear_up.join();
}
