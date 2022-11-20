#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod deduty;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
          deduty::package::commands::addLocalPackage
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
