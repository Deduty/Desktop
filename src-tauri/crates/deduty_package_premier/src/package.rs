use serde::Deserialize;

use deduty_package::package::DedutyPackage;

#[derive(Deserialize)]
struct PremierPackage {
    name: String,
    version: String,
    language: String
}

#[derive(Deserialize)]
struct PremierLectionExact {
    relative: String
}

#[derive(Deserialize)]
struct PremierLections {
    regex: Option<String>,
    exact: Option<Vec<PremierLectionExact>>
}


#[derive(Deserialize)]
pub struct DedutyPremierPackage {
    package: PremierPackage,
    lections: PremierLections
}

impl DedutyPackage for DedutyPremierPackage {
    fn name(&self) -> &String {
        &self.package.name
    }

    fn version(&self) -> &String {
        &self.package.version
    }

    fn language(&self) -> &String {
        &self.package.language
    }
}
