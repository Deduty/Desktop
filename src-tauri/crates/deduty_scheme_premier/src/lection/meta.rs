use deduty_package_traits::DedutyLectionMeta;


pub struct PremierLectionMeta {
    name: String,
    order: u64
}

impl DedutyLectionMeta for PremierLectionMeta {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn order(&self) -> u64 {
        self.order
    }
}

impl From<crate::schemes::lection::PremierLectionMeta> for PremierLectionMeta {
    fn from(schema: crate::schemes::lection::PremierLectionMeta) -> Self {
        Self {
            name: schema.name,
            order: schema.order
        }
    }
}
