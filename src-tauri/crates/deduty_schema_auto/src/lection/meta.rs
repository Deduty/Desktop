use deduty_package::MetaLection;

use super::AutoLection;


impl MetaLection for AutoLection {
    /// Json acceptable string
    fn meta(&self) -> Option<&str> {
        Some(self.meta.as_str())
    }

    /// Indicates how much space the lection occupies on the machine:
    /// - Use [`None`] if lection size **can't be** known without it's loading.
    /// - Use [`Some(0)`] when lection is not exist on the machine.
    /// - Otherwise [`Some(usize)`] where [`usize`] is the lection `bytes` size on the machine.
    /// 
    /// Note: This is not same as sum of lection files sizes.
    ///       This method indicates only lection size.
    fn size(&self) -> Option<usize> {
        Some(self.size)
    }
}
