#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#![feature(trait_upcasting)]

use std::sync::Arc;

use deduty_service::{ Service, StateFullService };
use xresult::XError;

mod commands;
mod managers;
mod utils;


async fn execute() -> tauri::App {
    // TODO: Provide logs or something for user when settings are not available
    let settings = Arc::new(managers::Settings::create().await.unwrap());

    // Managers setup
    let services = Arc::new(managers::ServiceManager::new());
    let readers = Arc::new(managers::ReaderManager::new());

    {
        let manager = &services;
        let mut services: Vec<Box<dyn Service>> = vec![];

        // Auto service
        services.push(deduty_schema_auto::service());

        // Premier service
        // let premier = Box::new(deduty_scheme_premier::service::PremierIndexService::new(packages_root.join("premier"))) as Box<dyn IndexService>;
        // packages.write().await.services_mut().insert("premier".to_string(), premier);

        for service in services {
            let service_id = service.id().to_string();
            let service_root = settings.services().join(&service_id);

            if let Err(error) = service.set_root(service_root.clone()).await {
                // TODO: Log error
                println!("While set root path of service `{service_id}`, next error occurred: {error}");
                println!("Skipping service with id {service_id}...");
                continue;
            }

            // Error may indicate that path already exist
            let _ = async_std::fs::create_dir_all(&service_root).await;
            

            match service.load_all(&service_root).await {
                Ok(_) => {
                    // ^----------------------------\
                    // TODO: Log all wrong entries -/

                    if let Err(error) = manager.add(service).await {
                        println!("Warning: Several packages have the same id `{service_id}`");
                        println!("{error}");
                        println!("Skipping service with id {service_id} (data will NOT be saved)...");
                    }
                },
                Err(error) => {
                    // TODO: Log error
                    // Note: This service must be unplugged since this thread is not main
                    //       so we can't interrupt initialization without join this thread
                    println!("While load all packages of service `{service_id}`, next error occurred: {error}");
                    println!("Skipping service with id {service_id} (data will NOT be saved)...");
                }
            }
        }
    }

    tauri::async_runtime::set(tokio::runtime::Handle::current());

    let mut application = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // CHUNKED
            self::commands::chunked::closeFileChunked,
            self::commands::chunked::getFileChunked,
            self::commands::chunked::openFileChunked,

            // WEB STORAGE
            self::commands::web_storage::webStorageExport,
            self::commands::web_storage::webStorageImport,
            self::commands::web_storage::webStorageClear,
            self::commands::web_storage::webStorageDelete,
            self::commands::web_storage::webStorageGet,
            self::commands::web_storage::webStorageSet,

            // SERVICE
            self::commands::service::getServiceAddRequirements,
            self::commands::service::getServiceUpdateRequirements,
            self::commands::service::listServices,

            // PACKAGE
            self::commands::package::addPackage,
            self::commands::package::getPackage,
            self::commands::package::listPackages,
            self::commands::package::subPackage,
            self::commands::package::updatePackage,
        ])
        .manage(settings.clone())
        .manage(services.clone())
        .manage(readers.clone())
        .build(tauri::generate_context!())
        .expect("Error while running tauri application");

    // Note: All magic goes here
    //       There is no another way to check if we PROBABLY need to close window
    while application.run_iteration().window_count != 0 {}

    {
        // SERVICES SAVING
        for service in services.list().await {
            let service_root = settings.services().join(service.borrow().id());

            let statefull = service.borrow() as &dyn StateFullService;
            let mut saving_issues = Vec::new();

            match statefull.save_all(&service_root).await {
                Ok(reasons) => {
                    for reason in reasons {
                        if let Err(reason) = reason {
                            saving_issues.push(reason);
                        }
                    }
                },
                Err(error) => {
                    println!("While saving service with id `{}` an error occurred: {error}", service.borrow().id());
                }
            }

            if !saving_issues.is_empty() {
                let error = XError::from((
                    "Internal error",
                    format!("While saving service with id `{}`, unable to save several packages:", service.borrow().id()),
                    saving_issues.as_slice()
                ));
                println!("{error}");
            }            
        }
    }

    application
}


/// ### Important
/// 
/// [`main`] function creates entire async runtime for property state management:
/// 1. Async resources \ state setup
/// 2. **Run application by loop**
/// 3. Async tear down
/// 
/// The second one, as you can see, allows to tear down without killing entire process
/// but we anyway MUST DO this, because tauri do additional clean on an application
/// 
fn main() {
    // Note: MUST HAVE
    //       https://github.com/tauri-apps/tauri/blob/dev/examples

    // Note: Allow use any libs (dlls)
    //       https://github.com/tauri-apps/tauri/blob/dev/examples/tauri-dynamic-lib/src-app1/src/main.rs

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Unable to create Tokio Runtime")
        .block_on(execute())
        //
        // Note: Exit through the tauri::process::exit with automatic cleanup
        .handle()
        .exit(0)
}
