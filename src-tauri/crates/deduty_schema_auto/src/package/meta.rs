use deduty_package::MetaPackage;

use super::AutoPackage;


impl MetaPackage for AutoPackage {
    /// Json acceptable string
    fn meta(&self) -> Option<&str> {
        self.meta.as_deref()
    }

    /// Indicates how much space the package occupies on the machine:
    /// - Use [`None`] if package size **can't be** known without it's loading.
    /// - Use [`Some(0)`] when package is not exist on the machine.
    /// - Otherwise [`Some(usize)`] where [`usize`] is the package `bytes` size on the machine.
    /// 
    /// Note: This is not same as sum of lection files sizes.
    ///       This method indicates only lection size.
    fn size(&self) -> Option<usize> {
        Some(self.size)
    }
}
