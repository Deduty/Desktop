use deduty_package::UniqueFile;

use super::AutoFile;


impl UniqueFile for AutoFile {
    fn id(&self) -> &str {
        self.id.as_str()
    }
}
