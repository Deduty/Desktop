use deduty_package::UniqueLection;

use super::AutoLection;


impl UniqueLection for AutoLection {
    fn id(&self) -> &str {
        self.id.as_str()
    }
}
