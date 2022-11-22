use deduty_package::lection::DedutyLection;

pub struct DedutyPremierLection {
    name: String
}

impl DedutyLection for DedutyPremierLection {
    fn name(&self) -> &String {
        &self.name
    }
}
