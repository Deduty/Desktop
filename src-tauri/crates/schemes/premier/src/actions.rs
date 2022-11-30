use std::error::Error;

use async_std::io::ReadExt;
use async_std::fs::File;

use package::package::traits::DedutyPackage;

use crate::schemes::package::PremierPackage as PremierPackageScheme;
use crate::package::PremierPackage;

pub async fn load(
    path: async_std::path::PathBuf
) -> Result<Box<dyn DedutyPackage>, Box<dyn Error>> {
    if !path.exists().await {
        return Err(format!("Path '{:#?}' is not exist", path).into());
    }
  	if !path.is_dir().await {
    	return Err(format!("Path '{:#?}' is not a directory", path).into());
  	}

    // PACKAGE
    let package = {
        let package_toml = path.join("package.toml");
        if !package_toml.exists().await {
            return Err("'package.toml' is not exist".into());
        }
        if !package_toml.is_file().await {
            return Err("'package.toml' is not a file".into());
        }

        let mut buffer = Vec::new();
        File::open(package_toml.as_path())
            .await
            .map_err(|error| error.to_string())?
            .read_to_end(&mut buffer)
            .await
            .map_err(|error| error.to_string())?;

        toml::from_slice::<PremierPackageScheme>(&buffer)?
    };

    Ok(Box::new(PremierPackage::from(package, &path)) as Box<dyn DedutyPackage>)
}
