use async_std::io::ReadExt;
use async_std::fs::File;

use deduty_package::package::traits::DedutyPackage;
use xresult::{ XError, XResult };

use crate::schemes::package::PremierPackage as PremierPackageScheme;
use crate::package::PremierPackage;

pub async fn load(
    path: async_std::path::PathBuf
) -> XResult<Box<dyn DedutyPackage>> {
    if !path.exists().await {
        return Err(Box::new(XError::from(("PremierLoadError", format!("Path '{:#?}' is not exist", path)))));
    }
  	if !path.is_dir().await {
        return Err(Box::new(XError::from(("PremierLoadError", format!("Path '{:#?}' is not a directory", path)))));
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

    let package = PremierPackage::from(package, &path).await?;
    Ok(Box::new(package) as Box<dyn DedutyPackage>)
}
