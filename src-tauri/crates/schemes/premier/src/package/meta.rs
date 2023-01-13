use deduty_package::package::traits::DedutyPackageMeta;

pub struct PremierPackageMeta {
    name: String,
    version: String,
    language: String,
    tags: Vec<String>
}

impl DedutyPackageMeta for PremierPackageMeta {
    fn name(&self) -> &String {
        &self.name
    }

    fn version(&self) -> &String {
        &self.version
    }

    fn language(&self) -> &String {
        &self.language
    }

    fn tags(&self) -> &Vec<String> {
        &self.tags
    }
}

impl From<crate::schemes::package::PackageMeta> for PremierPackageMeta {
    fn from(schema: crate::schemes::package::PackageMeta) -> Self {
        Self {
            name: schema.name,
            version: schema.version,
            language: schema.language,
            tags: schema.tags.unwrap_or(vec![])
        }
    }
}
