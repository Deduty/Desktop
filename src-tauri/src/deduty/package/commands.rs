#[tauri::command]
pub fn addLocalPackage(path: &str) -> String {
    path.to_owned()
}
