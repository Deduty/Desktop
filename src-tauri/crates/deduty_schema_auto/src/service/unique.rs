use deduty_service::UniqueService;

use super::AutoPackageService;


impl UniqueService for AutoPackageService {
    fn id(&self) -> &str {
        "auto"
    }
}
