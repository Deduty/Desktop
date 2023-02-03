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

        std::thread::spawn(move || {
            async_std::task::block_on(async move {
                let packages_root = settings.resources().join("services");

                // Premier service
                let premier = Box::new(deduty_scheme_premier::service::PremierIndexService::new(packages_root.join("premier"))) as Box<dyn IndexService>;
                packages.write().await.services_mut().insert("premier".to_string(), premier);
    
                // Load all
                let mut failures = Vec::new();
                for (key, service) in packages.write().await.services_mut().iter_mut() {
                    if let Err(error) = async_std::fs::create_dir_all(packages_root.join(key)).await {
                        println!("While initial folder creating, next error occurred: {error}");
                        panic!("Unable to continue on initial folder creating due to next error: {error}");
                    }

                    match service.load_all().await {
                        Ok(_) => { /* TODO: Log all wrong entries */ },
                        Err(error) => {
                            // TODO: Log error
                            // Note: This service must be unplugged since this thread is not main
                            //       so we can't interrupt initialization without join this thread
                            println!("While load all packages of service `{key}`, next error occurred: {error}");
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
            self::commands::package::subPackage,

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
        .manage(readers)
        .manage(storages.clone())
        .build(tauri::generate_context!())
        .expect("Error while running tauri application")
        .run(move |_handle, event| {
            let storages = storages.clone();
            let packages = packages.clone();

            if let tauri::RunEvent::Exit = event {
                let storages_save_thread = std::thread::spawn(move || {
                    async_std::task::block_on(async move {
                        let results = storages
                            .read()
                            .await
                            .save()
                            .await
                            .unwrap();

                        for error in results.iter().filter_map(|reason| reason.as_ref().err().map(|error| error.to_string())) {
                            println!("While save error occurred {error}")
                        }
                    })
                });

                let packages_save_thread = std::thread::spawn(move || {
                    async_std::task::block_on(async move {
                        for (key, service) in packages.write().await.services_mut().iter_mut() {
                            // TODO: Log errors
                            if let Err(error) = service.save_all().await {
                                println!("While service `{key}` saving, next error occurred: {error}");
                            }
                        }
                    })
                });

                // TODO: Log errors
                if let Err(error) = storages_save_thread.join() {
                    println!("While joining `storages save` thread, next error occurred: {error:#?}");
                }

                if let Err(error) = packages_save_thread.join() {
                    println!("While joining `packages save` thread, next error occurred: {error:#?}");
                }
            }
        });

    // TODO: This thread never join
    if let Err(error) = service_tear_up.join() {
        println!("While joining `services tear up` thread, next error occurred: {error:#?}");
    }
}
