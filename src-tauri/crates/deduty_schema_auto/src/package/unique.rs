use deduty_package::UniquePackage;

use super::AutoPackage;


impl UniquePackage for AutoPackage {
    fn id(&self) -> &str {
        self.id.as_str()
    }
}
