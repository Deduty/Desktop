use async_std::io::ReadExt;
use async_std::fs::File;
use async_std::path::Path;

use deduty_package::package::{
    DedutyPackage,
    SerializedDedutyPackage
};
use deduty_package_premier::package::DedutyPremierPackage;

#[tauri::command]
pub async fn addLocalPackage(path: &str) -> Result<SerializedDedutyPackage, String> {
    // PATH
	let path = Path::new(path);
    if !path.exists().await {
        return Err(format!("Path '{:#?}' is not exist", path));
    }
  	if !path.is_dir().await {
    	return Err(format!("Path '{:#?}' is not a directory", path));
  	}

    // PACKAGE
    let package_toml = path.join("package.toml");
	if !package_toml.exists().await {
    	return Err("'package.toml' is not exist".into());
  	}
	if !package_toml.is_file().await {
		return Err("'package.toml' is not a file".into());
	}

    let package_toml_content = {
        let mut buffer = Vec::new();
        File::open(package_toml.as_path())
            .await
            .map_err(|error| error.to_string())?
            .read_to_end(&mut buffer)
            .await
            .map_err(|error| error.to_string())?;
        buffer
    };

    // PACKAGE MANIFEST
    let manifest: super::manifest::PackageManifest = 
        toml::from_slice(&package_toml_content)
            .map_err(|error| error.to_string())?;

    match manifest.to_enum() {
        // PREMIER PACKAGE
        Some(super::manifest::PackageManifestVariants::Premier) => {
            let package = {
                let premier = toml::from_slice::<DedutyPremierPackage>(&package_toml_content)
                    .map_err(|error| error.to_string())?;
                Box::new(premier) as Box<dyn DedutyPackage>
            };

            Ok(package.into())
        },
        // UNKNOWN PACKAGE
        None => Err(format!("Manifest '{}' version is not supported", manifest.as_string()))
    }
}
