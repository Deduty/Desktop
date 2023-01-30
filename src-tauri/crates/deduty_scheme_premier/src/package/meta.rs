use deduty_package_traits::DedutyPackageMeta;

#[derive(Debug, Clone)]
pub struct PremierPackageMeta {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) language: String,
    pub(crate) tags: Vec<String>
}

impl DedutyPackageMeta for PremierPackageMeta {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn version(&self) -> String {
        self.version.clone()
    }

    fn language(&self) -> String {
        self.language.clone()
    }

    fn tags(&self) -> Vec<String> {
        self.tags.clone()
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
